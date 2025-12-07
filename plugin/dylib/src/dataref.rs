use crate::error::UdpRequestHandlerError;
use infra::udp::handler::{UdpRequestHandler, UdpRequestHandlerType};
use infra::udp::request::{RequestDataType, UdpRequest};
use std::fmt::Display;
use xplm::data::borrowed::DataRef;
use xplm::data::{ArrayRead, DataRead, DataType, ReadOnly};

pub(crate) struct DataRefReader;

impl DataRefReader {
    pub fn new() -> Self {
        Self {}
    }

    fn handle_numeric_data_ref<T>(data_ref: &str) -> Result<String, Box<dyn std::error::Error>>
    where
        T: DataType + Display,
        DataRef<T, ReadOnly>: DataRead<T>,
    {
        match DataRef::<T, ReadOnly>::find(data_ref) {
            Ok(data_ref_value) => Ok(format!("{}", data_ref_value.get())),
            Err(e) => Err(UdpRequestHandlerError::DataRefReadError {
                data_ref: data_ref.to_string(),
                cause: e,
            }
            .into()),
        }
    }

    fn handle_int_array_data_ref(data_ref: &str) -> Result<String, Box<dyn std::error::Error>> {
        match DataRef::<[i32], ReadOnly>::find(data_ref) {
            Ok(data_ref_value) => Ok(format!("{:?}", data_ref_value.as_vec())),
            Err(e) => Err(UdpRequestHandlerError::DataRefReadError {
                data_ref: data_ref.to_string(),
                cause: e,
            }
            .into()),
        }
    }

    fn handle_float_array_data_ref(data_ref: &str) -> Result<String, Box<dyn std::error::Error>> {
        match DataRef::<[f32], ReadOnly>::find(data_ref) {
            Ok(data_ref_value) => Ok(format!("{:?}", data_ref_value.as_vec())),
            Err(e) => Err(UdpRequestHandlerError::DataRefReadError {
                data_ref: data_ref.to_string(),
                cause: e,
            }
            .into()),
        }
    }
}

impl UdpRequestHandler for DataRefReader {
    fn get_handler_type(&self) -> UdpRequestHandlerType {
        UdpRequestHandlerType::DataRefReader
    }

    fn handle(&self, request: UdpRequest) -> Result<String, Box<dyn std::error::Error>> {
        let data_ref = request.get_data();
        match request.get_data_type() {
            RequestDataType::Int => Self::handle_numeric_data_ref::<i32>(data_ref.as_str()),
            RequestDataType::Float => Self::handle_numeric_data_ref::<f32>(data_ref.as_str()),
            RequestDataType::IntArray => Self::handle_int_array_data_ref(data_ref.as_str()),
            RequestDataType::FloatArray => Self::handle_float_array_data_ref(data_ref.as_str()),
        }
    }
}
