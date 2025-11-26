use crate::udp::message::dispatcher::MessageDispatcher;
use std::io::ErrorKind::{TimedOut, WouldBlock};
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tracing::{error, info};

struct UdpServer {
    running: Arc<AtomicBool>,
    server_thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    message_dispatcher: Arc<MessageDispatcher>,
}

impl UdpServer {
    fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            server_thread_handle: Arc::new(Mutex::new(None)),
            message_dispatcher: Arc::new(MessageDispatcher::new()),
        }
    }

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

        // blocking mode to avoid busy loop
        socket.set_nonblocking(false).expect("udp server failed to set non-blocking = false");

        // set read timeout to 100ms to ensure the server can stop gracefully
        let read_timeout = Some(Duration::from_millis(100));
        socket.set_read_timeout(read_timeout).expect("udp server failed to set read timeout");

        // create a buffer to store received data
        let mut buffer = [0u8; 2048];

        self.running.store(true, Ordering::SeqCst);

        info!("udp server listening on {} with blocking mode", addr);

        while self.running.load(Ordering::SeqCst) {
            match socket.recv_from(&mut buffer) {
                Ok((size, src)) => {
                    let message = std::str::from_utf8(&buffer[..size]).unwrap();
                    info!("udp server received message from {}: {:?}", src, message);

                    let dispatch_result = self.message_dispatcher.dispatch(message);
                    let response = dispatch_result.unwrap_or_else(|e| e.to_string());
                    info!("udp server sending response to {}: {:?}", src, response);

                    match socket.send_to(response.as_bytes(), src) {
                        Ok(_) => info!("udp server successfully sent response to {}", src),
                        Err(e) => error!("udp server failed to send response to {}: {:?}", src, e),
                    }
                }
                Err(ref e) if e.kind() == WouldBlock || e.kind() == TimedOut => {
                    // no data received, just continue to wait for next read
                    continue;
                }
                Err(e) => {
                    error!("udp server failed to receive data: {:?}", e);
                }
            }
        }

        info!("udp server gracefully stopped");
    }

    fn stop(&self) {
        info!("udp server gracefully stopping...");
        self.running.store(false, Ordering::SeqCst);
        if let Some(server_thread_handle) = self.server_thread_handle.lock().unwrap().take() {
            server_thread_handle.join().expect("udp server thread exited with error");
            info!("udp server thread exited successfully");
        }
    }
}

static UDP_SERVER: OnceLock<UdpServer> = OnceLock::new();

fn get_udp_server() -> &'static UdpServer {
    UDP_SERVER.get_or_init(UdpServer::new)
}

pub(crate) fn start(port: u16) {
    let udp_server = get_udp_server();
    let server_thread_handle = thread::Builder::new()
        .name("udp-server".to_string())
        .spawn(move || udp_server.start(port))
        .expect("udp server thread failed to spawn");
    *udp_server.server_thread_handle.lock().unwrap() = Some(server_thread_handle);
}

pub(crate) fn stop() {
    get_udp_server().stop();
}

#[cfg(test)]
mod tests {
    use crate::udp;
    use std::panic::catch_unwind;

    #[test]
    fn test_start_udp_server() {
        let port = 49000;
        let result = catch_unwind(|| udp::server::start(port));
        assert!(result.is_ok(), "test failed: udp server start should not panic");
    }

    #[test]
    fn test_stop_udp_server() {
        let result = catch_unwind(|| udp::server::stop());
        assert!(result.is_ok(), "test failed: udp server stop should not panic");
    }
}
