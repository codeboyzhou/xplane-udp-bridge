use crate::udp::UdpClient;
use nu_ansi_term::Color::{Cyan, Red, Yellow};

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

    /// Reads a dataref value as a float.
    ///
    /// This method sends a request to read the specified dataref and parses
    /// the response as a float value.
    ///
    /// # Arguments
    ///
    /// * `data_ref` - The dataref identifier to read (e.g., "sim/cockpit2/gauges/indicators/airspeed_kts_pilot")
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The parsed float value from the dataref
    /// * `Err(String)` - Error message if the request fails or parsing fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// let airspeed = reader.read_as_float("sim/cockpit2/gauges/indicators/airspeed_kts_pilot")?;
    /// println!("Current airspeed: {} knots", airspeed);
    /// ```
    pub(crate) fn read_as_float(&self, data_ref: &str) -> Result<f32, String> {
        let data = format!("dataref|read|float|{}", data_ref);
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
                let value = data.split("|").nth(2).unwrap().parse::<f32>().unwrap();
                Ok(value)
            }
            None => Err(Red.paint("no response from server").to_string()),
        }
    }
}
