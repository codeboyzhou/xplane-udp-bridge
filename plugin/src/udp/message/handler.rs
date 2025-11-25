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
    phantom_data: PhantomData<T>,
}

impl<T> DataRefReader<T>
where
    T: DataType + Debug + Send + Sync + 'static,
    DataRef<T, ReadOnly>: DataRead<T>,
{
    pub(crate) fn new() -> Self {
        Self { phantom_data: PhantomData }
    }
}

impl<T> MessageHandler for DataRefReader<T>
where
    T: DataType + Debug + Send + Sync + 'static,
    DataRef<T, ReadOnly>: DataRead<T>,
{
    fn handle(&self, src: SocketAddr, payload: &str, socket: &UdpSocket) {
        let type_name = std::any::type_name::<T>();
        info!("DataRefReader<{}> received message from {}: {}", type_name, src, payload);
        match DataRef::<T, ReadOnly>::find(payload) {
            Ok(dataref) => {
                let response = format!("{:?}", dataref.get());
                info!("DataRefReader<{}> read value {:?}", type_name, response);
                if let Err(e) = socket.send_to(response.as_bytes(), src) {
                    error!("DataRefReader<{}> failed to send response: {:?}", type_name, e);
                }
            }
            Err(e) => {
                let msg = format!("read error for {}: {:?}", payload, e);
                error!("DataRefReader<{}> {}", type_name, msg);
                if let Err(e) = socket.send_to(msg.as_bytes(), src) {
                    error!("DataRefReader<{}> failed to send error response: {:?}", type_name, e);
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
