//! UDP Request Handlers
//!
//! This module provides the core components for handling UDP requests in the X-Plane UDP bridge.
//! It defines the `RequestHandler` trait and the `DataRefReader` struct which is responsible
//! for reading data references from X-Plane.

use crate::error::RequestHandlerError;
use crate::udp::request::UdpRequest;
use std::fmt::Debug;
use std::marker::PhantomData;
use tracing::{debug, error};
use xplm::data::borrowed::DataRef;
use xplm::data::{DataRead, DataType, ReadOnly};

/// Trait for handling UDP requests.
///
/// This trait defines the interface that all request handlers must implement.
/// Handlers are responsible for processing incoming UDP requests and returning
/// appropriate responses or errors.
pub(crate) trait RequestHandler: Send + Sync + 'static {
    /// Handles an incoming UDP request.
    ///
    /// # Arguments
    ///
    /// * `request` - The UDP request to be processed
    ///
    /// # Returns
    ///
    /// A `Result` containing either the response string or a `RequestHandlerError`
    /// if the request could not be processed successfully.
    fn handle(&self, request: UdpRequest) -> Result<String, RequestHandlerError>;
}

/// A generic handler for reading X-Plane data references.
///
/// This struct implements the `RequestHandler` trait and provides functionality
/// to find and read data references from X-Plane. It is generic over the data type
/// of the data reference, allowing it to handle different types of data references
/// such as integers, floats, etc.
///
/// The `PhantomData<T>` field is used to indicate that the struct is generic over
/// type `T` without actually storing a value of that type.
pub(crate) struct DataRefReader<T> {
    phantom_data: PhantomData<T>,
}

impl<T> DataRefReader<T>
where
    T: DataType + Debug + Send + Sync + 'static,
    DataRef<T, ReadOnly>: DataRead<T>,
{
    /// Creates a new `DataRefReader` instance.
    ///
    /// # Returns
    ///
    /// A new `DataRefReader<T>` instance.
    pub(crate) fn new() -> Self {
        Self { phantom_data: PhantomData }
    }
}

impl<T> RequestHandler for DataRefReader<T>
where
    T: DataType + Debug + Send + Sync + 'static,
    DataRef<T, ReadOnly>: DataRead<T>,
{
    /// Handles a UDP request by finding and reading the specified X-Plane data reference.
    ///
    /// This method attempts to find the data reference specified in the request body,
    /// reads its current value, and returns the value as a string. If the data reference
    /// cannot be found or read, an appropriate error is returned.
    ///
    /// # Arguments
    ///
    /// * `request` - The UDP request containing the data reference name in its body
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - The formatted value of the data reference as a string
    /// - A `RequestHandlerError::DataRefFindError` if the data reference cannot be found
    ///
    /// # Examples
    ///
    /// ```rust
    /// let handler = DataRefReader::<f32>::new();
    /// let request = UdpRequest::new("sim/cockpit/gyros/ind_hdg_copilot_deg".to_string());
    /// match handler.handle(request) {
    ///     Ok(value) => println!("DataRef value: {}", value),
    ///     Err(e) => eprintln!("Error: {:?}", e),
    /// }
    /// ```
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
