use crate::error::UdpRequestHandlerError;
use infra::udp::handler::{UdpRequestHandler, UdpRequestHandlerType};
use infra::udp::request::UdpRequest;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use xplm::data::borrowed::DataRef;
use xplm::data::{DataRead, DataType, ReadOnly};

pub(crate) struct DataRefReader<T> {
    phantom_data: PhantomData<T>,
}

impl<T> DataRefReader<T> {
    pub(crate) fn new() -> Self {
        Self { phantom_data: PhantomData }
    }
}

impl<T> UdpRequestHandler for DataRefReader<T>
where
    T: DataType + Debug + Send + Sync + Display,
    DataRef<T, ReadOnly>: DataRead<T>,
{
    fn get_handler_type(&self) -> UdpRequestHandlerType {
        let data_type_name = std::any::type_name::<T>();
        match data_type_name {
            "i32" => UdpRequestHandlerType::IntDataRefReader,
            "f32" => UdpRequestHandlerType::FloatDataRefReader,
            _ => UdpRequestHandlerType::Unsupported,
        }
    }

    fn handle(&self, request: UdpRequest) -> Result<String, Box<dyn std::error::Error>> {
        let data_ref_key = request.get_data();
        match DataRef::<T>::find(data_ref_key.as_str()) {
            Ok(data_ref_value) => Ok(format!("{}", data_ref_value.get())),
            Err(e) => Err(UdpRequestHandlerError::DataRefReadError {
                data_ref: data_ref_key.to_string(),
                cause: e,
            }
            .into()),
        }
    }
}
