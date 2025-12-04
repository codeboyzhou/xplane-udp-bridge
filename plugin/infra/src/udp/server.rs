use crate::udp::error::UdpRequestHandlerError;
use crate::udp::handler::UdpRequestHandler;
use crate::udp::request::UdpRequest;
use crate::udp::response::Status::InternalServerError;
use crate::udp::response::{Status, UdpResponse};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, OnceLock};
use tokio::runtime::Runtime;
use tracing::{error, info};

struct UdpServer {
    request_handlers: Arc<Mutex<Vec<Box<dyn UdpRequestHandler>>>>,
}

impl UdpServer {
    fn new() -> Self {
        Self { request_handlers: Arc::new(Mutex::new(Vec::new())) }
    }

    fn new_tokio_runtime() -> Runtime {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(std::thread::available_parallelism().unwrap().get())
            .thread_name("tokio-async-udp-server-worker")
            .enable_all()
            .build()
            .unwrap()
    }

    fn start(self: Arc<Self>, port: u16) {
        // We spawn a background thread so X-Plane main thread is not blocked
        // and this server can continue to run even if the main thread is busy
        let thread_safe_server = self.clone();
        std::thread::spawn(move || {
            let addr = SocketAddr::from(([0, 0, 0, 0], port));
            let runtime = Self::new_tokio_runtime();
            runtime.block_on(async {
                let socket = match tokio::net::UdpSocket::bind(addr).await {
                    Ok(socket) => {
                        info!("UDP server successfully bound to {}", addr);
                        socket
                    }
                    Err(e) => {
                        error!("Failed to bind UDP server to {}: {:?}", addr, e);
                        return;
                    }
                };

                let mut buffer = vec![0u8; 2048];

                info!("UDP server started and listening on {}", addr);

                loop {
                    let (size, src) = match socket.recv_from(&mut buffer).await {
                        Ok((size, src)) => (size, src),
                        Err(e) => {
                            error!("UDP server failed to receive data: {:?}", e);
                            continue;
                        }
                    };

                    let message = match String::from_utf8(buffer[..size].to_vec()) {
                        Ok(message) => message,
                        Err(e) => {
                            let err_msg = format!("UDP server failed to parse message: {:?}", e);
                            error!("{}", err_msg);
                            let response = UdpResponse::error(Status::BadRequest, err_msg);
                            Self::send_response(&socket, response, src).await;
                            continue;
                        }
                    };

                    let request = match UdpRequest::new(message) {
                        Ok(request) => request,
                        Err(e) => {
                            let err_msg = format!("UDP server failed to parse request: {:?}", e);
                            error!("{}", err_msg);
                            let response = UdpResponse::error(Status::BadRequest, err_msg);
                            Self::send_response(&socket, response, src).await;
                            continue;
                        }
                    };

                    match thread_safe_server.handle_request(request) {
                        Ok(response) => {
                            Self::send_response(&socket, UdpResponse::ok(response), src).await
                        }
                        Err(e) => {
                            let err_msg = format!("UDP server failed to handle request: {:?}", e);
                            error!("{}", err_msg);
                            let response = UdpResponse::error(InternalServerError, err_msg);
                            Self::send_response(&socket, response, src).await
                        }
                    }
                }
            });
        });
    }

    fn handle_request(&self, request: UdpRequest) -> Result<String, Box<dyn std::error::Error>> {
        for handler in self.request_handlers.lock().unwrap().iter() {
            if handler.get_handler_type() == request.determine_handler_type() {
                return handler.handle(request);
            }
        }
        Err(UdpRequestHandlerError::NoHandlerFound { request }.into())
    }

    async fn send_response(socket: &tokio::net::UdpSocket, response: UdpResponse, src: SocketAddr) {
        if let Err(e) = socket.send_to(response.serialize().as_bytes(), src).await {
            error!("UDP server failed to send response: {:?}", e);
        }
    }
}

static SERVER: OnceLock<Arc<UdpServer>> = OnceLock::new();

fn get_udp_server() -> &'static Arc<UdpServer> {
    SERVER.get_or_init(|| Arc::new(UdpServer::new()))
}

pub fn register_request_handler(handler: Box<dyn UdpRequestHandler + Send + Sync>) {
    info!("Registering UDP request handler: {:?}", handler.get_handler_type());
    get_udp_server().request_handlers.lock().unwrap().push(handler);
}

pub fn start(port: u16) {
    get_udp_server().clone().start(port);
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
}
