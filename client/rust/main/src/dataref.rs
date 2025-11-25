use crate::UdpClient;

pub(crate) struct Reader<'a> {
    udp_client: &'a UdpClient,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(udp_client: &'a UdpClient) -> Self {
        Self { udp_client }
    }

    pub(crate) fn read_as_float(&self, dataref: &str) -> Result<f32, String> {
        let data = format!("dataref|read|float|{}", dataref);
        println!("➡️ Sending dataref read request: {}", data);

        let resp = self.udp_client.send_and_recv(data.as_bytes());
        match resp {
            Some(resp_body_as_bytes) => {
                let data = std::str::from_utf8(resp_body_as_bytes.as_slice()).unwrap();
                let value = data
                    .parse::<f32>()
                    .map_err(|e| format!("❌ Failed to parse dataref value: {:?}", e))?;
                println!("⬅️ Received dataref value: {}", value);
                Ok(value)
            }
            None => {
                let err_msg = format!("❌ No response from server or unknown dataref: {}", dataref);
                eprintln!("{}", err_msg);
                Err(err_msg)
            }
        }
    }
}
