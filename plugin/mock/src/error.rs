#[derive(Debug, thiserror::Error)]
pub(crate) enum MockUdpRequestHandlerError {
    #[error("DataRefReadError: {} is not found in mock_data_refs", data_ref)]
    DataRefReadError { data_ref: String },
}
