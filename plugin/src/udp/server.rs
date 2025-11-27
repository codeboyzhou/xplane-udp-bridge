//! # UDP Server Module
//!
//! This module provides UDP server functionality for the X-Plane UDP Bridge plugin.
//! It handles incoming UDP requests, dispatches them to appropriate handlers,
//! and sends responses back to clients.

use crate::udp::dispatcher::RequestDispatcher;
use crate::udp::request::UdpRequest;
use crate::udp::response::{Status, UdpResponse};
use std::io::ErrorKind::{TimedOut, WouldBlock};
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tracing::{debug, error, info};

/// A UDP server that listens for incoming requests and handles them.
///
/// The server runs in a separate thread and can be started and stopped
/// dynamically. It uses a request dispatcher to process incoming requests
/// and send responses back to clients.
struct UdpServer {
    /// Atomic flag indicating whether the server is currently running
    running: Arc<AtomicBool>,
    /// Handle to the server thread, protected by a mutex
    server_thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    /// Request dispatcher for processing incoming requests
    request_dispatcher: Arc<RequestDispatcher>,
}

impl UdpServer {
    /// Creates a new UDP server instance.
    ///
    /// # Returns
    ///
    /// Returns a new `UdpServer` instance with default settings.
    fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            server_thread_handle: Arc::new(Mutex::new(None)),
            request_dispatcher: Arc::new(RequestDispatcher::new()),
        }
    }

    /// Starts the UDP server on the specified port.
    ///
    /// This method binds to the specified port, configures the socket,
    /// and enters a loop to receive and process requests. The method
    /// runs in a blocking fashion until the server is stopped.
    ///
    /// # Arguments
    ///
    /// * `port` - The UDP port to listen on
    fn start(&self, port: u16) {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        if self.running.load(Ordering::SeqCst) {
            info!("udp server is already running on {}", addr);
            return;
        }

        let socket = match UdpSocket::bind(addr) {
            Ok(socket) => {
                info!("udp server successfully bound to {}", addr);
                socket
            }
            Err(e) => {
                error!("udp server failed to bind to {}: {:?}", addr, e);
                return;
            }
        };

        // set blocking mode to avoid server busy loop
        match socket.set_nonblocking(false) {
            Ok(_) => info!("udp server successfully set blocking mode to avoid busy loop"),
            Err(e) => error!("udp server failed to set blocking mode: {:?}", e),
        }

        // set read timeout to 100ms to ensure the server can stop gracefully
        let read_timeout = Some(Duration::from_millis(100));
        match socket.set_read_timeout(read_timeout) {
            Ok(_) => info!("udp server successfully set read timeout to {:?}", read_timeout),
            Err(e) => error!("udp server failed to set read timeout: {:?}", e),
        }

        // create a buffer to store received data
        let mut buffer = [0u8; 2048];

        self.running.store(true, Ordering::SeqCst);

        info!("udp server listening on {} with blocking mode", addr);

        while self.running.load(Ordering::SeqCst) {
            self.run(&socket, &mut buffer);
        }

        info!("udp server gracefully stopped");
    }

    /// Handles a single iteration of the server's main loop.
    ///
    /// This method attempts to receive data from the socket. If data is
    /// received, it is parsed and dispatched to the appropriate handler.
    /// If no data is received or an error occurs, the method returns
    /// immediately.
    ///
    /// If a request is successfully received and dispatched, a response
    /// is sent back to the client. If an error occurs during request
    /// dispatching or response sending, an error response is sent back
    /// to the client.
    ///
    /// # Arguments
    ///
    /// * `socket` - The UDP socket to use for receiving data
    /// * `buffer` - A mutable buffer to store received data
    fn run(&self, socket: &UdpSocket, buffer: &mut [u8]) {
        let socket_recv_result = socket.recv_from(buffer);

        if socket_recv_result.is_err() {
            let e = socket_recv_result.err().unwrap();
            if e.kind() == WouldBlock || e.kind() == TimedOut {
                // no data received, nothing to do, just continue to wait for next read
                return;
            }
            // other errors, log and continue to wait for next read
            error!("udp server failed to receive data: {:?}", e);
            return;
        }

        let (size, src) = socket_recv_result.unwrap();

        if size == 0 {
            debug!("udp server received empty message from {}", src);
            let message = "empty message".to_string();
            let response = UdpResponse::error(Status::BadRequest, message);
            self.send_response(socket, src, response);
            return;
        }

        let message_decode_result = std::str::from_utf8(&buffer[..size]);
        if message_decode_result.is_err() {
            let e = message_decode_result.err().unwrap();
            let err = format!("udp server failed to parse message from {}: {:?}", src, e);
            error!("{}", err);
            let response = UdpResponse::error(Status::BadRequest, err);
            self.send_response(socket, src, response);
            return;
        }

        let message = message_decode_result.unwrap();
        debug!("udp server received message from {}: {:?}", src, message);

        let udp_request_build_result = UdpRequest::from_str(message);
        if udp_request_build_result.is_err() {
            let e = udp_request_build_result.err().unwrap();
            error!("udp server failed to build request from message due to error: {:?}", e);
            let response = UdpResponse::error(Status::BadRequest, e.to_string());
            self.send_response(socket, src, response);
            return;
        }

        let request = udp_request_build_result.unwrap();
        let response = match self.request_dispatcher.dispatch(request) {
            Ok(response) => UdpResponse::ok(response),
            Err(e) => UdpResponse::error(Status::BadRequest, e.to_string()),
        };
        self.send_response(socket, src, response);
    }

    /// Sends a response back to the client.
    ///
    /// This method serializes the `UdpResponse` into a string and sends
    /// it back to the specified client address.
    ///
    /// # Arguments
    ///
    /// * `socket` - The UDP socket to use for sending the response
    /// * `src` - The client address to send the response to
    /// * `response` - The `UdpResponse` to send back to the client
    fn send_response(&self, socket: &UdpSocket, src: SocketAddr, response: UdpResponse) {
        match socket.send_to(response.serialize().as_bytes(), src) {
            Ok(_) => debug!("udp server successfully sent response to {}", src),
            Err(e) => error!("udp server failed to send response to {}: {:?}", src, e),
        }
    }

    /// Stops the UDP server gracefully.
    ///
    /// This method sets the running flag to false and waits for the
    /// server thread to exit cleanly.
    fn stop(&self) {
        info!("udp server gracefully stopping...");
        self.running.store(false, Ordering::SeqCst);
        if let Some(server_thread_handle) = self.server_thread_handle.lock().unwrap().take() {
            server_thread_handle.join().expect("udp server thread exited with error");
            info!("udp server thread exited successfully");
        }
    }
}

/// Global instance of the UDP server, initialized once.
///
/// This uses the `OnceLock` pattern to ensure that only one instance
/// of the UDP server is created and used throughout the application.
static UDP_SERVER: OnceLock<UdpServer> = OnceLock::new();

/// Gets the global UDP server instance, creating it if necessary.
///
/// # Returns
///
/// Returns a reference to the global `UdpServer` instance.
fn get_udp_server() -> &'static UdpServer {
    UDP_SERVER.get_or_init(UdpServer::new)
}

/// Starts the UDP server on the specified port in a new thread.
///
/// This is a public function that creates a new thread for the server
/// and starts it on the specified port. The server will run until
/// `stop()` is called.
///
/// # Arguments
///
/// * `port` - The UDP port to listen on
pub(crate) fn start(port: u16) {
    let udp_server = get_udp_server();
    let server_thread_handle = thread::Builder::new()
        .name("udp-server".to_string())
        .spawn(move || udp_server.start(port))
        .expect("udp server thread failed to spawn");
    *udp_server.server_thread_handle.lock().unwrap() = Some(server_thread_handle);
}

/// Stops the UDP server.
///
/// This is a public function that stops the running UDP server
/// and waits for it to exit cleanly.
pub(crate) fn stop() {
    get_udp_server().stop();
}

#[cfg(test)]
mod tests {
    use crate::udp;
    use std::panic::catch_unwind;

    /// Tests that starting the UDP server does not panic.
    ///
    /// This test verifies that the server can be started without
    /// causing a panic, which would indicate a critical error.
    #[test]
    fn test_start_udp_server() {
        let port = 49000;
        let result = catch_unwind(|| udp::server::start(port));
        assert!(result.is_ok(), "test failed: udp server start should not panic");
    }

    /// Tests that stopping the UDP server does not panic.
    ///
    /// This test verifies that the server can be stopped without
    /// causing a panic, which would indicate a critical error.
    #[test]
    fn test_stop_udp_server() {
        let result = catch_unwind(|| udp::server::stop());
        assert!(result.is_ok(), "test failed: udp server stop should not panic");
    }
}
