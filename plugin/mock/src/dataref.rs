use crate::error::MockUdpRequestHandlerError;
use infra::udp::handler::{UdpRequestHandler, UdpRequestHandlerType};
use infra::udp::request::UdpRequest;
use std::collections::HashMap;
use tracing::{error, info};

pub(crate) struct MockDataRefReader {
    mock_data_refs: HashMap<&'static str, &'static str>,
}

impl MockDataRefReader {
    pub(crate) fn new() -> Self {
        let mock_data_refs = HashMap::from([
            ("sim/cockpit2/controls/parking_brake_ratio", "1.0"),
            ("sim/cockpit2/engine/actuators/throttle_ratio", "0.5"),
            ("sim/cockpit2/engine/actuators/eng_master", "[0,1]"),
            ("sim/cockpit2/electrical/battery_on", "[1,1,1]"),
        ]);
        Self { mock_data_refs }
    }
}

impl UdpRequestHandler for MockDataRefReader {
    fn get_handler_type(&self) -> UdpRequestHandlerType {
        UdpRequestHandlerType::DataRefReader
    }

    fn handle(&self, request: UdpRequest) -> Result<String, Box<dyn std::error::Error>> {
        info!("Handling request: {:?}", request);
        let data_ref_key = request.get_data();
        match self.mock_data_refs.get(data_ref_key.as_str()) {
            Some(value) => {
                info!("DataRef {} found with value {}", data_ref_key, value);
                Ok(value.to_string())
            }
            None => {
                error!("DataRef {} not found in mock_data_refs", data_ref_key);
                Err(Box::new(MockUdpRequestHandlerError::DataRefReadError {
                    data_ref: data_ref_key.clone(),
                }))
            }
        }
    }
}
