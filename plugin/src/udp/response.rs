//! UDP Response module for handling and formatting response messages.
//!
//! This module provides structures and functionality for creating and serializing
//! UDP response messages that are sent back to clients. It follows a simple HTTP-like
//! response format with status codes and messages.

use crate::udp::request::UdpRequest;

/// Represents the status of a UDP response.
///
/// This enum defines the possible status codes that can be included in a response,
/// similar to HTTP status codes but simplified for UDP communication.
pub(crate) enum Status {
    /// Indicates the request was processed successfully (HTTP 200 equivalent)
    Ok,
    /// Indicates the request was malformed or invalid (HTTP 400 equivalent)
    BadRequest,
    /// Indicates the server encountered an internal error (HTTP 500 equivalent)
    InternalServerError,
}

/// Represents a complete UDP response that can be sent to clients.
///
/// This structure combines a status code with a body message to form a complete
/// response that can be serialized and transmitted over UDP.
pub(crate) struct UdpResponse {
    /// The status code indicating the result of the request processing
    status: Status,
    /// The actual message content to be sent to the client
    message: String,
}

impl UdpResponse {
    /// Creates a successful response with the specified message.
    ///
    /// # Arguments
    /// * `message` - A string containing the success message to be sent to the client
    ///
    /// # Returns
    /// A new `UdpResponse` instance with `Status::Ok` and the provided message
    ///
    /// # Examples
    /// ```
    /// let response = UdpResponse::ok("Data received successfully".to_string());
    /// ```
    pub(crate) fn ok(message: String) -> Self {
        Self { status: Status::Ok, message }
    }

    /// Creates an error response with the specified status and message.
    ///
    /// # Arguments
    /// * `status` - The error status to include in the response
    /// * `message` - A string containing the error message to be sent to the client
    ///
    /// # Returns
    /// A new `UdpResponse` instance with the provided status and message
    ///
    /// # Examples
    /// ```
    /// let response = UdpResponse::error(Status::BadRequest, "Invalid message format".to_string());
    /// ```
    pub(crate) fn error(status: Status, message: String) -> Self {
        Self { status, message }
    }

    /// Serializes the response into a string format suitable for UDP transmission.
    ///
    /// This method converts the response into a string format that follows the
    /// message protocol defined in `MessageFormat`. The format is:
    /// "CODE|PHRASE|MESSAGE" where:
    /// - CODE: Numeric status code (200 for OK, 400 for Bad Request)
    /// - PHRASE: Textual status description ("OK" or "Bad Request")
    /// - MESSAGE: The actual message content
    ///
    /// # Returns
    /// A string representation of the response ready for UDP transmission
    ///
    /// # Examples
    /// ```
    /// let response = UdpResponse::ok("Success".to_string());
    /// let serialized = response.serialize();
    /// // Result: "200|OK|Success"
    /// ```
    pub(crate) fn serialize(&self) -> String {
        let UdpResponse { status, message } = self;
        let code = match status {
            Status::Ok => 200,
            Status::BadRequest => 400,
            Status::InternalServerError => 500,
        };
        let phrase = match status {
            Status::Ok => "OK",
            Status::BadRequest => "Bad Request",
            Status::InternalServerError => "Internal Server Error",
        };
        let message_parts = [code.to_string(), phrase.to_string(), message.clone()];
        message_parts.join(UdpRequest::MESSAGE_PARTS_SEPARATOR)
    }
}
