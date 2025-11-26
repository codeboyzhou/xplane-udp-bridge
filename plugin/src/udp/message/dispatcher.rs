use crate::error::UdpMessageError;
use crate::udp::message::format::MessageFormat;
use crate::udp::message::handler::{DataRefReader, MessageHandler, MessageHandlerSelector};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::RwLock;
use tracing::{error, info};

pub(crate) struct MessageDispatcher {
    lockable_message_handlers: RwLock<HashMap<String, Box<dyn MessageHandler>>>,
}

impl MessageDispatcher {
    pub(crate) fn new() -> Self {
        let mut message_handlers: HashMap<String, Box<dyn MessageHandler>> = HashMap::new();
        message_handlers.insert(
            MessageHandlerSelector::DATAREF_READ_INT.to_string(),
            Box::new(DataRefReader::<i32>::new()),
        );
        message_handlers.insert(
            MessageHandlerSelector::DATAREF_READ_FLOAT.to_string(),
            Box::new(DataRefReader::<f32>::new()),
        );
        Self { lockable_message_handlers: RwLock::new(message_handlers) }
    }

    pub(crate) fn dispatch(&self, message: &str) -> Result<String, UdpMessageError> {
        info!("udp server dispatching message: {}", message);

        let message_format_result = MessageFormat::from_str(message);
        if message_format_result.is_err() {
            let err = message_format_result.err().unwrap();
            error!("failed to dispatch message due to message format parse error: {:?}", err);
            return Err(UdpMessageError::FormatError { source: err });
        }

        let message_format = message_format_result.unwrap();
        let message_handler_selector = message_format.parse_message_handler_selector();
        let message_handlers = self.lockable_message_handlers.read().unwrap();

        if let Some(message_handler) = message_handlers.get(&message_handler_selector) {
            info!("udp server handling message: {}", message);
            match message_handler.handle(&message_format.data) {
                Ok(response) => Ok(response),
                Err(e) => {
                    error!("udp server handle message error: {:?}", e);
                    Err(UdpMessageError::HandlerError { source: e })
                }
            }
        } else {
            error!("no message handler impl found for message: {}", message);
            Err(UdpMessageError::HandlerNotFound { message: message.to_string() })
        }
    }
}
