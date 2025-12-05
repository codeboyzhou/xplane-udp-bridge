use std::io;
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::time::Duration;

/// UDP client for communicating with the XPlane UDP bridge plugin.
///
/// This struct encapsulates the UDP socket and server address needed for
/// communication with the XPlane UDP bridge plugin. It provides basic
/// functionality for sending data and receiving responses.
pub(crate) struct UdpClient {
    /// Server address (e.g., "127.0.0.1:49000")
    server_addr: String,

    /// UDP socket for communication with the server
    socket: UdpSocket,
}

impl UdpClient {
    /// Creates a new UDP client instance.
    ///
    /// # Arguments
    ///
    /// * `host` - Server IP address (e.g., "127.0.0.1")
    /// * `port` - Server port (e.g., 49000)
    /// * `timeout_secs` - Socket timeout in seconds (e.g., 30)
    ///
    /// # Returns
    ///
    /// * `Ok(UdpClient)` - Successfully created client instance
    /// * `Err(io::Error)` - Error information if creation fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = UdpClient::new("127.0.0.1", 49000, 30)?;
    /// ```
    pub(crate) fn new(host: &str, port: u16, timeout_secs: u64) -> io::Result<Self> {
        println!("🔗 Connecting to {}:{} with timeout {} seconds", host, port, timeout_secs);

        let server_addr = format!("{}:{}", host, port);

        // Bind to local random port for client socket
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        // Set socket read timeout
        socket.set_read_timeout(Some(Duration::from_secs(timeout_secs)))?;

        println!("✅  Connected successfully via UDP protocol");

        Ok(Self { server_addr, socket })
    }

    /// Sends data and waits for a response.
    ///
    /// # Arguments
    ///
    /// * `data` - Byte data to send
    ///
    /// # Returns
    ///
    /// * `Some(Vec<u8>)` - Response data received on success
    /// * `None` - Timeout or any error occurred
    ///
    /// # Examples
    ///
    /// ```rust
    /// let response = client.send_and_recv(&[0x01, 0x02, 0x03]);
    /// if let Some(data) = response {
    ///     println!("Received: {:?}", data);
    /// }
    /// ```
    pub(crate) fn send_and_recv(&self, data: &[u8]) -> Option<Vec<u8>> {
        // Send data
        if let Err(e) = self.socket.send_to(data, &self.server_addr) {
            eprintln!("❌ UDP error while sending data: {}", e);
            return None;
        }

        let mut buffer = [0u8; 2048];

        // Wait for UDP response
        match self.socket.recv_from(&mut buffer) {
            Ok((size, _src)) => Some(buffer[..size].to_vec()),
            Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                let timeout = self.socket.read_timeout().unwrap().unwrap().as_secs();
                println!("⏰ UDP request timed out after {} seconds", timeout);
                None
            }
            Err(e) => {
                eprintln!("❌ UDP error while receiving data: {}", e);
                None
            }
        }
    }
}
