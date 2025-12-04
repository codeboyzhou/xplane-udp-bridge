use crate::udp::error::InvalidUdpRequestError;
use crate::udp::error::InvalidUdpRequestError::{
    InvalidMessageFormat, MismatchedDataType, UnrecognizedRequestType, UnsupportedOperation,
};
use crate::udp::handler::UdpRequestHandlerType;

#[derive(Debug)]
pub enum RequestType {
    DataRef,
}

impl RequestType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestType::DataRef => "dataref",
        }
    }
}

#[derive(Debug)]
pub enum RequestOperation {
    Read,
}

impl RequestOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestOperation::Read => "read",
        }
    }
}

#[derive(Debug)]
pub enum RequestDataType {
    Int,
    Float,
}

impl RequestDataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestDataType::Int => "int",
            RequestDataType::Float => "float",
        }
    }
}

#[derive(Debug)]
pub struct UdpRequest {
    request_type: RequestType,
    operation: RequestOperation,
    data_type: RequestDataType,
    data: String,
}

impl UdpRequest {
    pub const MESSAGE_PARTS_SEPARATOR: &'static str = "|";

    const MESSAGE_PARTS_COUNT: usize = 4;

    pub(crate) fn new(message: String) -> Result<Self, InvalidUdpRequestError> {
        let parts: Vec<&str> = message.split(Self::MESSAGE_PARTS_SEPARATOR).collect();

        if parts.len() != Self::MESSAGE_PARTS_COUNT {
            return Err(InvalidMessageFormat { message });
        }

        Ok(Self {
            request_type: match parts[0] {
                "dataref" => RequestType::DataRef,
                _ => return Err(UnrecognizedRequestType { request_type: parts[0].to_string() }),
            },
            operation: match parts[1] {
                "read" => RequestOperation::Read,
                _ => return Err(UnsupportedOperation { operation: parts[1].to_string() }),
            },
            data_type: match parts[2] {
                "int" => RequestDataType::Int,
                "float" => RequestDataType::Float,
                _ => return Err(MismatchedDataType { data_type: parts[2].to_string() }),
            },
            data: parts[3].to_string(),
        })
    }

    pub(crate) fn determine_handler_type(&self) -> UdpRequestHandlerType {
        match (&self.request_type, &self.operation, &self.data_type) {
            (RequestType::DataRef, RequestOperation::Read, RequestDataType::Int) => {
                UdpRequestHandlerType::IntDataRefReader
            }
            (RequestType::DataRef, RequestOperation::Read, RequestDataType::Float) => {
                UdpRequestHandlerType::FloatDataRefReader
            }
        }
    }

    pub fn get_data(&self) -> String {
        self.data.clone()
    }
}
