use thiserror::Error;
use xplm::data::borrowed::FindError;

#[derive(Error, Debug)]
pub(crate) enum PluginError {}

#[derive(Error, Debug)]
pub(crate) enum UdpMessageError {
    #[error("udp server handle message error: {source}")]
    FormatError {
        #[source]
        source: MessageFormatError,
    },

    #[error("no message handler impl found for message: {message}")]
    HandlerNotFound { message: String },

    #[error("udp server handle message error: {source}")]
    HandlerError {
        #[source]
        source: MessageHandlerError,
    },
}

#[derive(Error, Debug)]
pub(crate) enum MessageFormatError {
    #[error("invalid message format: {message}")]
    InvalidMessageFormat { message: String },
}

#[derive(Error, Debug)]
pub(crate) enum MessageHandlerError {
    #[error("failed to find dataref [{dataref}]: {source}")]
    DataRefFindError {
        dataref: String,
        #[source]
        source: FindError,
    },
}
