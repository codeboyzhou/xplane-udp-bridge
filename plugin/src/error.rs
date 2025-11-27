//! Error Types for the X-Plane UDP Bridge Plugin
//!
//! This module defines the error types used throughout the X-Plane UDP bridge plugin.
//! It provides specific error enums for different components of the system, including
//! plugin errors, bad request errors, and request handler errors.

use crate::udp::request::UdpRequest;
use thiserror::Error;
use xplm::data::borrowed::FindError;

/// Enumeration of plugin-level errors.
///
/// This enum is currently empty but reserved for future plugin-specific errors.
/// It implements the standard Error and Debug traits for proper error handling.
#[derive(Error, Debug)]
pub(crate) enum PluginError {}

/// Enumeration of errors related to malformed or invalid UDP requests.
///
/// This enum represents errors that occur when parsing or validating incoming
/// UDP requests. It provides specific error variants for different types of
/// request format issues.
#[derive(Error, Debug)]
pub(crate) enum BadRequestError {
    /// Error variant for invalid message format.
    ///
    /// This error is returned when the incoming message does not conform to
    /// the expected format: "request_type|method|data_type|body".
    ///
    /// # Fields
    ///
    /// * `message` - The raw message that failed to parse
    #[error("invalid message format: {message}")]
    InvalidMessageFormat { message: String },
}

/// Enumeration of errors related to request handling.
///
/// This enum represents errors that occur during the processing of valid
/// UDP requests. It includes errors for missing handlers and data reference
/// lookup failures.
#[derive(Error, Debug)]
pub(crate) enum RequestHandlerError {
    /// Error variant for when no handler implementation is found for a request.
    ///
    /// This error is returned when the dispatcher cannot locate an appropriate
    /// handler for the given request type, method, and data type combination.
    ///
    /// # Fields
    ///
    /// * `request` - The request for which no handler was found
    #[error("no request handler impl found for request: {:?}", request)]
    HandlerImplNotFound { request: UdpRequest },

    /// Error variant for failures when finding a data reference.
    ///
    /// This error is returned when attempting to locate a data reference
    /// in X-Plane fails, typically because the data reference does not exist
    /// or is not accessible.
    ///
    /// # Fields
    ///
    /// * `dataref` - The name of the data reference that could not be found
    /// * `source` - The underlying FindError from the XPLM library
    #[error("failed to find dataref [{dataref}]: {source}")]
    DataRefFindError {
        dataref: String,
        #[source]
        source: FindError,
    },
}
