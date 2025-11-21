use std::io;
use std::io::{ErrorKind, Write};
use std::net::UdpSocket;
use std::time::Duration;

/// UDP Client for XPlane UDP bridge plugin.
struct UdpClient {
    server_addr: String,
    socket: UdpSocket,
}

impl UdpClient {
    /// Initialize UDP Client for XPlane UDP bridge plugin.
    ///
    /// Args:
    ///     host: server IP (e.g., "127.0.0.1")
    ///     port: server port (e.g., 49000)
    ///     timeout: socket timeout seconds (e.g., 3.0)
    ///
    /// Returns:
    ///     UdpClient instance or error on failure
    fn new(host: &str, port: u16, timeout: f64) -> io::Result<Self> {
        println!(
            "🔌 connecting to {}:{} with timeout {} seconds",
            host, port, timeout
        );

        let server_addr = format!("{}:{}", host, port);

        // Bind to local random port
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        // Set socket read timeout
        socket.set_read_timeout(Some(Duration::from_secs_f64(timeout)))?;

        println!("✅ connected successfully via UDP protocol");

        Ok(Self {
            server_addr,
            socket,
        })
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
        if let Err(e) = self.socket.send_to(data, &self.server_addr) {
            eprintln!("❌ UDP error while sending: {}", e);
            return None;
        }

        let mut buf = [0u8; 2048];

        // Wait for UDP response
        match self.socket.recv_from(&mut buf) {
            Ok((size, _src)) => Some(buf[..size].to_vec()),
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

    /// Close the socket.
    ///
    /// (Dropping the socket automatically closes it)
    fn close(self) {
        println!("UDP client closed");
    }
}

fn main() {
    // Create UDP client
    let client = UdpClient::new("127.0.0.1", 49000, 3.0).expect("Failed to create UDP client");

    // Send 5 test messages to server
    for i in 0..5 {
        let msg = format!("hello message {}", i);

        println!("➡️ sending message {}: {}", i, msg);

        let resp = client.send_and_recv(msg.as_bytes());

        // Print result
        match resp {
            Some(r) => println!("⬅️ received message {}: {:?}", i, r),
            None => println!("⚠️ no response from server for message {}", i),
        }

        // sleep for 1 second between requests to avoid overwhelming the server
        std::thread::sleep(Duration::from_secs(1));
    }

    // Close client
    client.close();

    // Avoid immediate exit
    print!("Press ENTER to exit...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}
