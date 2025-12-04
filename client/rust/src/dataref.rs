use crate::UdpClient;

pub(crate) struct DataRefReader<'a> {
    udp_client: &'a UdpClient,
}

impl<'a> DataRefReader<'a> {
    pub(crate) fn new(udp_client: &'a UdpClient) -> Self {
        Self { udp_client }
    }

    pub(crate) fn read_as_float(&self, data_ref: &str) -> Result<f32, String> {
        let data = format!("dataref|read|float|{}", data_ref);
        println!("➡️ Sending dataref read request: {}", data);

        match self.udp_client.send_and_recv(data.as_bytes()) {
            Some(response_body_as_bytes) => {
                let data = match std::str::from_utf8(response_body_as_bytes.as_slice()) {
                    Ok(data) => {
                        println!("⬅️ Received dataref response body: {}", data);
                        data
                    }
                    Err(e) => {
                        let err_msg = format!("❌ Failed to parse response body as UTF-8: {:?}", e);
                        eprintln!("{}", err_msg);
                        return Err(err_msg);
                    }
                };
                let value = data
                    .split("|")
                    .nth(2)
                    .unwrap_or("0.0")
                    .parse::<f32>()
                    .map_err(|e| format!("❌ Failed to parse dataref value: {:?}", e))?;
                println!("⬅️ Parsed dataref value: {}", value);
                Ok(value)
            }
            None => {
                let err_msg = "❌ No response from server";
                eprintln!("{}", err_msg);
                Err(err_msg.to_string())
            }
        }
    }
}
