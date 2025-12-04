use xplm::data::borrowed::FindError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum UdpRequestHandlerError {
    #[error("UDP request handler failed to read data ref: {}, caused by: {:?}", data_ref, cause)]
    DataRefReadError {
        data_ref: String,
        #[source]
        cause: FindError,
    },
}
