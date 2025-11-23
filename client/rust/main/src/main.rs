mod dataref;

use std::io;
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::time::Duration;

/// UDP Client for XPlane UDP bridge plugin.
struct UdpClient {
    // Server address (e.g., "127.0.0.1:49000")
    server_addr: String,

    // UDP socket for communication with server
    socket: UdpSocket,
}

impl UdpClient {
    /// Initialize UDP Client for XPlane UDP bridge plugin.
    ///
    /// Args:
    ///     host: server IP (e.g., "127.0.0.1")
    ///     port: server port (e.g., 49000)
    ///     timeout_secs: socket timeout seconds (e.g., 3.0)
    ///
    /// Returns:
    ///     UdpClient instance or error on failure
    fn new(host: &str, port: u16, timeout_secs: f64) -> io::Result<Self> {
        println!(
            "🔌 Creating UDP client to server {}:{} with timeout {} seconds",
            host, port, timeout_secs
        );

        let server_addr = format!("{}:{}", host, port);

        // Bind to local random port for client socket
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        // Set socket read timeout
        socket.set_read_timeout(Some(Duration::from_secs_f64(timeout_secs)))?;

        println!("✅ UDP client created successfully and bound to {}", socket.local_addr()?);

        Ok(Self { server_addr, socket })
    }

    /// Send bytes and wait for response.
    ///
    /// Args:
    ///     data: bytes to send
    ///
    /// Returns:
    ///     Some(Vec<u8>) on success
    ///     None on timeout or any error
    fn send_and_recv(&self, data: &[u8]) -> Option<Vec<u8>> {
        // Send data
        match self.socket.send_to(data, &self.server_addr) {
            Ok(_) => println!("✅ UDP data sent successfully, waiting for response..."),
            Err(e) => eprintln!("❌ UDP error while sending: {}", e),
        }

        let mut buffer = [0u8; 2048];

        // Wait for UDP response
        match self.socket.recv_from(&mut buffer) {
            Ok((size, _src)) => Some(buffer[..size].to_vec()),
            Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                let timeout = self.socket.read_timeout().unwrap().unwrap().as_secs_f64();
                println!("⏱ UDP request timed out after {} seconds", timeout);
                None
            }
            Err(e) => {
                eprintln!("❌ UDP error while receiving: {}", e);
                None
            }
        }
    }
}

fn main() {
    // Create UDP client
    let client = UdpClient::new("127.0.0.1", 49000, 3.0).expect("Failed to create UDP client");

    // Create dataref reader
    let dataref_reader = dataref::Reader::new(&client);
    
    loop {
        // Read dataref value examples
        match dataref_reader.read_as_float("sim/cockpit2/controls/parking_brake_ratio") {
            Ok(value) => println!("⬅️ received dataref value: {}", value),
            Err(err_msg) => eprintln!("Error reading dataref: {}", err_msg),
        }

        // Sleep for a short duration to avoid overloading the server
        std::thread::sleep(Duration::from_millis(1000));
    }
}
