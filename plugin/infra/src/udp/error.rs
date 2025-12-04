use crate::udp::request::UdpRequest;

#[derive(Debug, thiserror::Error)]
pub(crate) enum InvalidUdpRequestError {
    #[error("UDP request message format is invalid: {}", message)]
    InvalidMessageFormat { message: String },

    #[error("UDP request type is unrecognized: {}", request_type)]
    UnrecognizedRequestType { request_type: String },

    #[error("UDP request operation is unsupported: {}", operation)]
    UnsupportedOperation { operation: String },

    #[error("UDP request data type is mismatched: {}", data_type)]
    MismatchedDataType { data_type: String },
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum UdpRequestHandlerError {
    #[error("No UDP request handler impl found for request: {:?}", request)]
    NoHandlerImplFound { request: UdpRequest },

    #[error("UDP server failed to try lock request handlers")]
    TryLockError,
}
