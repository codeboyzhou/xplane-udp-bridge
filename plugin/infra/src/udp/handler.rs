use crate::udp::request::UdpRequest;

pub trait UdpRequestHandler: Send + Sync {
    fn get_handler_type(&self) -> UdpRequestHandlerType;
    fn handle(&self, request: UdpRequest) -> Result<String, Box<dyn std::error::Error>>;
}

#[derive(Debug, PartialEq)]
pub enum UdpRequestHandlerType {
    Unsupported,
    DataRefReader,
}
