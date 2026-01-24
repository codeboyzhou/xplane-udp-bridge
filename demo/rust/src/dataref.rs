use crate::udp::UdpClient;
use nu_ansi_term::Color::{Cyan, Red, Yellow};
use uuid::Uuid;

/// A reader for X-Plane data references (datarefs) via UDP communication.
///
/// This struct provides functionality to read dataref values from X-Plane
/// through the UDP bridge plugin. It uses a UDP client to send requests
/// and parse the responses.
pub(crate) struct DataRefReader<'a> {
    /// The UDP client used for communication with the X-Plane UDP bridge
    udp_client: &'a UdpClient,
}

impl<'a> DataRefReader<'a> {
    /// Creates a new DataRefReader instance.
    ///
    /// # Arguments
    ///
    /// * `udp_client` - A reference to the UDP client for communication
    ///
    /// # Returns
    ///
    /// A new DataRefReader instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let reader = DataRefReader::new(&udp_client);
    /// ```
    pub(crate) fn new(udp_client: &'a UdpClient) -> Self {
        Self { udp_client }
    }

    /// Reads a dataref value as a string.
    ///
    /// # Arguments
    ///
    /// * `data_ref` - The dataref identifier to read
    /// * `type_str` - The type of the dataref value, e.g., "int", "float", "[int]", "[float]"
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The parsed value from the dataref as a string
    /// * `Err(String)` - Error message if the request fails or parsing fails
    pub(crate) fn read(&self, data_ref: &str, type_str: &str) -> Result<String, String> {
        let request_id = Uuid::new_v4().simple().to_string();
        let data = format!("{}|dataref|read|{}|{}", request_id, type_str, data_ref);

        println!("{}", "=".repeat(100));
        println!("{}", Cyan.paint(format!("Sending dataref read request: {}", data)));

        match self.udp_client.send_and_recv(data.as_bytes()) {
            Some(response_body_as_bytes) => {
                let data = match std::str::from_utf8(response_body_as_bytes.as_slice()) {
                    Ok(data) => {
                        println!(
                            "{}",
                            Yellow.paint(format!("Received dataref read response body: {}", data))
                        );
                        data
                    }
                    Err(e) => {
                        let msg = Red.paint(format!("Failed to parse response body: {:?}", e));
                        eprintln!("{}", msg);
                        return Err(msg.to_string());
                    }
                };

                match data.split("|").nth(3) {
                    Some(value_str) => Ok(value_str.to_string()),
                    None => {
                        let msg = Red.paint(format!("Failed to parse dataref value: {}", data));
                        eprintln!("{}", msg);
                        Err(msg.to_string())
                    }
                }
            }
            None => Err(Red.paint("no response from server").to_string()),
        }
    }
}
