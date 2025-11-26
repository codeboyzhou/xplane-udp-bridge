use crate::error::MessageHandlerError;
use std::fmt::Debug;
use std::marker::PhantomData;
use tracing::{error, info};
use xplm::data::borrowed::DataRef;
use xplm::data::{DataRead, DataType, ReadOnly};

pub(crate) trait MessageHandler: Send + Sync + 'static {
    fn handle(&self, data: &str) -> Result<String, MessageHandlerError>;
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
    fn handle(&self, data: &str) -> Result<String, MessageHandlerError> {
        let handler_type = format!("DataRefReader<{}>", std::any::type_name::<T>());
        info!("{} finding dataref: {}", handler_type, data);
        match DataRef::<T, ReadOnly>::find(data) {
            Ok(dataref) => {
                let value = format!("{:?}", dataref.get());
                info!("{} found dataref [{}] and read the value: {}", handler_type, data, value);
                Ok(value)
            }
            Err(e) => {
                error!("{} failed to find dataref [{}]: {:?}", handler_type, data, e);
                Err(MessageHandlerError::DataRefFindError { dataref: data.to_string(), source: e })
            }
        }
    }
}

pub(crate) struct MessageHandlerSelector;

impl MessageHandlerSelector {
    pub(crate) const DATAREF_READ_INT: &'static str = "dataref|read|int";

    pub(crate) const DATAREF_READ_FLOAT: &'static str = "dataref|read|float";
}
