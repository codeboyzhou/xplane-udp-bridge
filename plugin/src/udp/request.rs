//! UDP Request Parsing and Handling
//!
//! This module provides structures and functionality for parsing and handling UDP requests
//! in the X-Plane UDP bridge. It defines the request format, request types, methods,
//! and data types, as well as the main `UdpRequest` struct that represents an incoming
//! UDP request.

use crate::error::BadRequestError;
use std::str::FromStr;
use tracing::debug;

/// Enumeration of supported request types.
///
/// This enum defines the types of requests that can be processed by the UDP bridge.
#[derive(Debug)]
enum RequestType {
    /// Request to access X-Plane data references
    DataRef,
}

/// Enumeration of supported request methods.
///
/// This enum defines the operations that can be performed on the requested data.
#[derive(Debug)]
enum RequestMethod {
    /// Read operation to retrieve data
    Read,
}

/// Enumeration of supported data types.
///
/// This enum defines the types of data that can be requested from X-Plane.
#[derive(Debug)]
enum DataType {
    /// Integer data type
    Int,
    /// Float data type
    Float,
}

/// Represents a UDP request received by the server.
///
/// This struct encapsulates all the information needed to process a UDP request,
/// including the request type, method, data type, and the request body.
///
/// The request format is: "request_type|method|data_type|body"
/// Example: "dataref|read|int|sim/cockpit/gyros/ind_hdg_copilot_deg"
#[derive(Debug)]
pub(crate) struct UdpRequest {
    /// The type of request (e.g., DataRef)
    request_type: RequestType,
    /// The method to be applied (e.g., Read)
    method: RequestMethod,
    /// The data type of the requested value (e.g., Int, Float)
    data_type: DataType,
    /// The body of the request, typically containing the data reference name
    body: String,
}

impl UdpRequest {
    /// Separator used to split message parts in the request format
    pub(crate) const MESSAGE_PARTS_SEPARATOR: &'static str = "|";

    /// Expected number of parts in a properly formatted message
    const MESSAGE_SPLIT_PARTS: usize = 4;

    /// Returns the body of the request.
    ///
    /// # Returns
    ///
    /// A string slice containing the request body, typically the data reference name.
    pub(crate) fn body(&self) -> &str {
        self.body.as_str()
    }

    /// Parses and returns a handler selector string based on the request components.
    ///
    /// This method constructs a string that can be used to select the appropriate
    /// handler for this request. The format is "request_type|method|data_type".
    ///
    /// # Returns
    ///
    /// A string representing the handler selector.
    ///
    /// # Examples
    ///
    /// For a request with type DataRef, method Read, and data type Int,
    /// this method returns "dataref|read|int".
    pub(crate) fn parse_handler_selector(&self) -> String {
        let request_type = match self.request_type {
            RequestType::DataRef => "dataref",
        };
        let method = match self.method {
            RequestMethod::Read => "read",
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

    /// Parses a string message into a UdpRequest.
    ///
    /// This method attempts to parse a string in the format "request_type|method|data_type|body"
    /// into a UdpRequest struct. If the format is invalid or contains unknown values,
    /// it returns a BadRequestError.
    ///
    /// # Arguments
    ///
    /// * `message` - The string message to be parsed
    ///
    /// # Returns
    ///
    /// A Result containing either the parsed UdpRequest or a BadRequestError if parsing fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let message = "dataref|read|int|sim/cockpit/gyros/ind_hdg_copilot_deg";
    /// match UdpRequest::from_str(message) {
    ///     Ok(request) => println!("Successfully parsed request"),
    ///     Err(e) => eprintln!("Failed to parse request: {:?}", e),
    /// }
    /// ```
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
                "read" => RequestMethod::Read,
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
