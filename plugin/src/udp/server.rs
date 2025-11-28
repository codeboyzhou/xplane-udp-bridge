//! # UDP Server Module
//!
//! This module provides UDP server functionality for the X-Plane UDP Bridge plugin.
//! It handles incoming UDP requests, dispatches them to appropriate handlers,
//! and sends responses back to clients.

use crate::udp::dispatcher::RequestDispatcher;
use crate::udp::request::UdpRequest;
use crate::udp::response::{Status, UdpResponse};
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::UdpSocket;
use tracing::{error, info};

/// A UDP server that listens for incoming requests and handles them.
///
/// The server runs in a separate thread and can be started and stopped
/// dynamically. It uses a request dispatcher to process incoming requests
/// and send responses back to clients.
pub(crate) struct UdpServer;

impl UdpServer {
    /// Starts the UDP server on the specified port.
    ///
    /// This method binds to the specified port, configures the socket,
    /// and enters a loop to receive and process requests. The method
    /// runs in a blocking fashion until the server is stopped.
    ///
    /// # Arguments
    ///
    /// * `port` - The UDP port to listen on
    pub(crate) fn start(port: u16) {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let dispatcher = RequestDispatcher::new();

        // We spawn a background thread so X-Plane main thread is not blocked
        // and this server can continue to run even if the main thread is busy
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(std::thread::available_parallelism().unwrap().get())
                .thread_name("tokio-async-udp-server-worker")
                .enable_all()
                .build()
                .unwrap();

            runtime.block_on(async move {
                let socket = match UdpSocket::bind(addr).await {
                    Ok(socket) => {
                        info!("udp server successfully bound to {}", addr);
                        socket
                    }
                    Err(e) => {
                        error!("udp server failed to bind to {}: {:?}", addr, e);
                        return;
                    }
                };

                let mut buffer = vec![0u8; 2048];

                loop {
                    let (size, src) = match socket.recv_from(&mut buffer).await {
                        Ok((size, src)) => (size, src),
                        Err(e) => {
                            error!("udp server failed to receive data: {:?}", e);
                            continue;
                        }
                    };

                    let message = match String::from_utf8(buffer[..size].to_vec()) {
                        Ok(message) => message,
                        Err(e) => {
                            let err = format!("udp server failed to parse message: {:?}", e);
                            error!("{}", err);
                            let response = UdpResponse::error(Status::BadRequest, err);
                            Self::send_response(&socket, src, response).await;
                            continue;
                        }
                    };

                    let request = match UdpRequest::from_str(&message) {
                        Ok(request) => request,
                        Err(e) => {
                            let err = format!("udp server failed to build request: {:?}", e);
                            error!("{}", err);
                            let response = UdpResponse::error(Status::BadRequest, err);
                            Self::send_response(&socket, src, response).await;
                            continue;
                        }
                    };

                    let response = match dispatcher.dispatch(request) {
                        Ok(response) => response,
                        Err(e) => {
                            let err = format!("udp server failed to handle request: {:?}", e);
                            error!("{}", err);
                            let response = UdpResponse::error(Status::InternalServerError, err);
                            Self::send_response(&socket, src, response).await;
                            continue;
                        }
                    };

                    Self::send_response(&socket, src, UdpResponse::ok(response)).await;
                }
            });
        });
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
    async fn send_response(socket: &UdpSocket, src: SocketAddr, response: UdpResponse) {
        if let Err(e) = socket.send_to(response.serialize().as_bytes(), src).await {
            error!("udp server failed to send response to {}: {:?}", src, e);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::udp::server::UdpServer;
    use std::panic::catch_unwind;

    /// Tests that starting the UDP server does not panic.
    ///
    /// This test verifies that the server can be started without
    /// causing a panic, which would indicate a critical error.
    #[test]
    fn test_start_udp_server() {
        let port = 49000;
        let result = catch_unwind(|| UdpServer::start(port));
        assert!(result.is_ok(), "test failed: udp server start should not panic");
    }
}
