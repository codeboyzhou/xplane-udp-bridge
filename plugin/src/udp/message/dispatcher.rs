use crate::udp::message::handler::{DataRefReader, MessageHandler, MessageHandlerSelector};
use crate::udp::message::spec::MessageSpec;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
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

    pub(crate) fn dispatch(&self, src: SocketAddr, message: &str, socket: &UdpSocket) {
        let message_spec_result = MessageSpec::from_str(message);
        if message_spec_result.is_err() {
            let err_msg = message_spec_result.err().unwrap();
            error!("failed to dispatch message due to message spec parse error: {}", err_msg);
            return;
        }

        let message_spec = message_spec_result.unwrap();
        let message_handler_selector = message_spec.get_message_handler_selector();
        let message_handlers = self.lockable_message_handlers.read().unwrap();

        if let Some(message_handler) = message_handlers.get(&message_handler_selector) {
            info!("dispatching message to handler selector: {}", message_handler_selector);
            message_handler.handle(src, &message_spec.payload, socket);
        } else {
            error!("no message handler impl found for selector: {}", message_handler_selector);
        }
    }
}
