use std::io::ErrorKind::{TimedOut, WouldBlock};
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tracing::{error, info};

trait MessageHandler: Send + Sync + 'static {
    fn handle(&self, src: SocketAddr, bytes: &[u8]);
}

struct DefaultMessageHandler;

impl MessageHandler for DefaultMessageHandler {
    fn handle(&self, src: SocketAddr, bytes: &[u8]) {
        let data = std::str::from_utf8(bytes).unwrap();
        info!("udp default message handler received from {}: {}", src, data);
    }
}

struct UdpServer {
    running: Arc<AtomicBool>,
    server_thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    message_handler: Arc<dyn MessageHandler>,
}

impl UdpServer {
    fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            server_thread_handle: Arc::new(Mutex::new(None)),
            message_handler: Arc::new(DefaultMessageHandler {}),
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
                    self.message_handler.handle(src, &buffer[..size]);
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
    UDP_SERVER.get_or_init(|| UdpServer::new())
}

pub(crate) fn start_udp_server(port: u16) {
    let udp_server = get_udp_server();
    let server_thread_handle = thread::Builder::new()
        .name("udp-server".to_string())
        .spawn(move || udp_server.start(port))
        .expect("udp server thread failed to spawn");
    *udp_server.server_thread_handle.lock().unwrap() = Some(server_thread_handle);
}

pub(crate) fn stop_udp_server() {
    get_udp_server().stop();
}

#[cfg(test)]
mod tests {
    use crate::udp;
    use std::panic::catch_unwind;

    #[test]
    fn test_start_udp_server() {
        let port = 49000;
        let result = catch_unwind(|| udp::start_udp_server(port));
        assert!(result.is_ok(), "test failed: udp server start should not panic");
    }

    #[test]
    fn test_stop_udp_server() {
        let result = catch_unwind(|| udp::stop_udp_server());
        assert!(result.is_ok(), "test failed: udp server stop should not panic");
    }
}
