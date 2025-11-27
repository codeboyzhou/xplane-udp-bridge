use crate::error::BadRequestError;
use std::str::FromStr;
use tracing::debug;

#[derive(Debug)]
enum RequestType {
    DataRef,
}

#[derive(Debug)]
enum UdpMethod {
    Read,
}

#[derive(Debug)]
enum DataType {
    Int,
    Float,
}

#[derive(Debug)]
pub(crate) struct UdpRequest {
    request_type: RequestType,
    method: UdpMethod,
    data_type: DataType,
    body: String,
}

impl UdpRequest {
    pub(crate) const MESSAGE_PARTS_SEPARATOR: &'static str = "|";

    const MESSAGE_SPLIT_PARTS: usize = 4;

    pub(crate) fn body(&self) -> &str {
        self.body.as_str()
    }

    pub(crate) fn parse_handler_selector(&self) -> String {
        let request_type = match self.request_type {
            RequestType::DataRef => "dataref",
        };
        let method = match self.method {
            UdpMethod::Read => "read",
        };
        let data_type = match self.data_type {
            DataType::Int => "int",
            DataType::Float => "float",
        };
        let handler_selector = [request_type, method, data_type];
        handler_selector.join(Self::MESSAGE_PARTS_SEPARATOR)
    }
}

impl FromStr for UdpRequest {
    type Err = BadRequestError;

    fn from_str(message: &str) -> Result<Self, Self::Err> {
        debug!("udp server building request from message: {}", message);

        let parts = message.split(Self::MESSAGE_PARTS_SEPARATOR).collect::<Vec<&str>>();
        let err = BadRequestError::InvalidMessageFormat { message: message.to_string() };

        if parts.len() != Self::MESSAGE_SPLIT_PARTS {
            return Err(err);
        }

        Ok(Self {
            request_type: match parts[0] {
                "dataref" => RequestType::DataRef,
                _ => return Err(err),
            },
            method: match parts[1] {
                "read" => UdpMethod::Read,
                _ => return Err(err),
            },
            data_type: match parts[2] {
                "int" => DataType::Int,
                "float" => DataType::Float,
                _ => return Err(err),
            },
            body: parts[3].to_string(),
        })
    }
}
