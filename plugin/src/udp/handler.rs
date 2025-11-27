use crate::error::RequestHandlerError;
use crate::udp::request::UdpRequest;
use std::fmt::Debug;
use std::marker::PhantomData;
use tracing::{debug, error};
use xplm::data::borrowed::DataRef;
use xplm::data::{DataRead, DataType, ReadOnly};

pub(crate) trait RequestHandler: Send + Sync + 'static {
    fn handle(&self, request: UdpRequest) -> Result<String, RequestHandlerError>;
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

impl<T> RequestHandler for DataRefReader<T>
where
    T: DataType + Debug + Send + Sync + 'static,
    DataRef<T, ReadOnly>: DataRead<T>,
{
    fn handle(&self, request: UdpRequest) -> Result<String, RequestHandlerError> {
        let handler_type = format!("DataRefReader<{}>", std::any::type_name::<T>());
        let dataref = request.body();
        debug!("{} finding dataref: {}", handler_type, dataref);
        match DataRef::<T, ReadOnly>::find(dataref) {
            Ok(dataref_value_wrapper) => {
                let value = format!("{:?}", dataref_value_wrapper.get());
                debug!("{} found dataref [{}] and read value: {}", handler_type, dataref, value);
                Ok(value)
            }
            Err(e) => {
                error!("{} failed to find dataref [{}]: {:?}", handler_type, dataref, e);
                Err(RequestHandlerError::DataRefFindError {
                    dataref: dataref.to_string(),
                    source: e,
                })
            }
        }
    }
}
