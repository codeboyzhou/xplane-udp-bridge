use crate::error::MockUdpRequestHandlerError;
use infra::udp::handler::{UdpRequestHandler, UdpRequestHandlerType};
use infra::udp::request::UdpRequest;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use tracing::{error, info};

pub(crate) struct MockDataRefReader<T> {
    phantom_data: PhantomData<T>,
    mock_data_refs: HashMap<&'static str, &'static str>,
}

impl<T> MockDataRefReader<T> {
    pub(crate) fn new() -> Self {
        let mut mock_data_refs = HashMap::new();
        mock_data_refs.insert("sim/cockpit2/controls/parking_brake_ratio", "0.5");
        Self { phantom_data: PhantomData, mock_data_refs }
    }
}

impl<T> UdpRequestHandler for MockDataRefReader<T>
where
    T: Debug + Send + Sync,
{
    fn get_handler_type(&self) -> UdpRequestHandlerType {
        let data_type_name = std::any::type_name::<T>();
        match data_type_name {
            "i32" => UdpRequestHandlerType::IntDataRefReader,
            "f32" => UdpRequestHandlerType::FloatDataRefReader,
            _ => UdpRequestHandlerType::Unsupported,
        }
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
