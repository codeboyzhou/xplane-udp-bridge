use crate::error::RequestHandlerError;
use crate::udp::handler::{DataRefReader, RequestHandler};
use crate::udp::request::UdpRequest;
use std::collections::HashMap;
use std::sync::RwLock;
use tracing::{debug, error};

pub(crate) struct RequestDispatcher {
    lockable_request_handlers: RwLock<HashMap<String, Box<dyn RequestHandler>>>,
}

impl RequestDispatcher {
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
