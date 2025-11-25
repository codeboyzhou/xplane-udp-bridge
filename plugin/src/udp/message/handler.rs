use std::fmt::Debug;
use std::marker::PhantomData;
use std::net::{SocketAddr, UdpSocket};
use tracing::{error, info};
use xplm::data::borrowed::DataRef;
use xplm::data::{DataRead, DataType, ReadOnly};

pub(crate) trait MessageHandler: Send + Sync + 'static {
    fn handle(&self, src: SocketAddr, payload: &str, socket: &UdpSocket);
}

pub(crate) struct DataRefReader<T> {
    type_selector: &'static str,
    phantom_data: PhantomData<T>,
}

impl<T> DataRefReader<T>
where
    T: DataType + Debug + Send + Sync + 'static,
    DataRef<T, ReadOnly>: DataRead<T>,
{
    pub(crate) fn new(type_selector: &'static str) -> Self {
        Self { type_selector, phantom_data: PhantomData }
    }
}

impl<T> MessageHandler for DataRefReader<T>
where
    T: DataType + Debug + Send + Sync + 'static,
    DataRef<T, ReadOnly>: DataRead<T>,
{
    fn handle(&self, src: SocketAddr, payload: &str, socket: &UdpSocket) {
        info!("{} received message from {}: {}", self.type_selector, src, payload);
        match DataRef::<T, ReadOnly>::find(payload) {
            Ok(dataref) => {
                let response = format!("{:?}", dataref.get());
                info!("{} read value {:?}", self.type_selector, response);
                if let Err(e) = socket.send_to(response.as_bytes(), src) {
                    error!("{} failed to send response: {:?}", self.type_selector, e);
                }
            }
            Err(e) => {
                let msg = format!("dataref read error for {}: {:?}", payload, e);
                error!("{}", msg);
                if let Err(e) = socket.send_to(msg.as_bytes(), src) {
                    error!("{} failed to send error response: {:?}", self.type_selector, e);
                }
            }
        }
    }
}

pub(crate) struct MessageHandlerSelector;

impl MessageHandlerSelector {
    pub(crate) const DATAREF_READ_INT: &'static str = "dataref|read|int";

    pub(crate) const DATAREF_READ_FLOAT: &'static str = "dataref|read|float";
}
