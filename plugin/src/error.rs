use crate::udp::request::UdpRequest;
use thiserror::Error;
use xplm::data::borrowed::FindError;

#[derive(Error, Debug)]
pub(crate) enum PluginError {}

#[derive(Error, Debug)]
pub(crate) enum BadRequestError {
    #[error("invalid message format: {message}")]
    InvalidMessageFormat { message: String },
}

#[derive(Error, Debug)]
pub(crate) enum RequestHandlerError {
    #[error("no request handler impl found for request: {:?}", request)]
    HandlerImplNotFound { request: UdpRequest },

    #[error("failed to find dataref [{dataref}]: {source}")]
    DataRefFindError {
        dataref: String,
        #[source]
        source: FindError,
    },
}
