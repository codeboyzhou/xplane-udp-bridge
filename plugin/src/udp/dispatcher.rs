//! Request dispatcher module for routing UDP requests to appropriate handlers.
//!
//! This module provides the `RequestDispatcher` struct which is responsible for
//! registering request handlers and dispatching incoming UDP requests to the
//! appropriate handler based on the request type, method, and data type.

use crate::error::RequestHandlerError;
use crate::udp::handler::{DataRefReader, RequestHandler};
use crate::udp::request::UdpRequest;
use std::collections::HashMap;
use std::sync::RwLock;
use tracing::{debug, error};

/// A dispatcher that routes UDP requests to the appropriate handlers.
///
/// The `RequestDispatcher` maintains a registry of request handlers indexed by
/// a selector string that represents the request type, method, and data type.
/// It provides thread-safe access to handlers using a `RwLock` to allow
/// concurrent read operations.
pub(crate) struct RequestDispatcher {
    /// A thread-safe map of handler selectors to their corresponding request handlers
    lockable_request_handlers: RwLock<HashMap<String, Box<dyn RequestHandler>>>,
}

impl RequestDispatcher {
    /// Creates a new `RequestDispatcher` with default handlers registered.
    ///
    /// This constructor initializes the dispatcher with handlers for:
    /// - Integer dataref reading (`dataref|read|int`)
    /// - Float dataref reading (`dataref|read|float`)
    ///
    /// # Returns
    /// A new `RequestDispatcher` instance with pre-registered handlers
    ///
    /// # Examples
    /// ```
    /// let dispatcher = RequestDispatcher::new();
    /// ```
    pub(crate) fn new() -> Self {
        let mut request_handlers: HashMap<String, Box<dyn RequestHandler>> = HashMap::new();
        request_handlers.insert(
            ["dataref", "read", "int"].join(UdpRequest::MESSAGE_PARTS_SEPARATOR),
            Box::new(DataRefReader::<i32>::new()),
        );
        request_handlers.insert(
            ["dataref", "read", "float"].join(UdpRequest::MESSAGE_PARTS_SEPARATOR),
            Box::new(DataRefReader::<f32>::new()),
        );
        Self { lockable_request_handlers: RwLock::new(request_handlers) }
    }

    /// Dispatches a UDP request to the appropriate handler.
    ///
    /// This method parses the request to determine the appropriate handler selector,
    /// finds the corresponding handler, and delegates the request processing to it.
    ///
    /// # Arguments
    /// * `request` - The UDP request to be dispatched
    ///
    /// # Returns
    /// * `Ok(String)` - The response string from the handler if successful
    /// * `Err(RequestHandlerError)` - An error if no handler is found or if the handler fails
    ///
    /// # Examples
    /// ```
    /// let dispatcher = RequestDispatcher::new();
    /// let request = UdpRequest::from_str("dataref|read|int|sim/cockpit2/engine/actuators/throttle_ratio_all")?;
    /// match dispatcher.dispatch(request) {
    ///     Ok(response) => println!("Response: {}", response),
    ///     Err(e) => eprintln!("Error: {:?}", e),
    /// }
    /// ```
    pub(crate) fn dispatch(&self, request: UdpRequest) -> Result<String, RequestHandlerError> {
        debug!("udp server dispatching request: {:?}", request);
        let request_handler_selector = request.parse_handler_selector();
        let request_handlers = self.lockable_request_handlers.read().unwrap();
        match request_handlers.get(&request_handler_selector) {
            Some(request_handler) => {
                debug!("udp server handling request: {:?}", request);
                match request_handler.handle(request) {
                    Ok(response) => Ok(response),
                    Err(e) => Err(e),
                }
            }
            None => {
                error!("no request handler impl found for request: {:?}", request);
                Err(RequestHandlerError::HandlerImplNotFound { request })
            }
        }
    }
}
