use crate::{Body, Request, Response, Result};

/// The `Middleware` trait is implemented by middleware structs.
///
/// It is used to intercept requests and responses. And optionally to modify them.
///
/// Example:
///
/// ```
/// use swim::prelude::*;
///
/// #[derive(Debug)]
/// struct Logger;
///
/// impl Middleware for Logger {
///     fn pre(&self, request: Request<Body>) -> Result<Response<Body>> {
///         println!("New request: {:?}", request.uri());
///
///         Ok(request)
///     }
///
///     fn post(&self, request: Request<Body>, response: Response<Body>) -> Result<Response<Body>> {
///         println!("Response: {:?}", response.status());
///
///         Ok(response)
///     }
/// }
/// ```
///
/// Default implementations simply return the request and response without modifying them. You can override these methods to provide custom implementations.
#[async_trait::async_trait]
pub trait Middleware: std::fmt::Debug + Send + Sync + 'static {
    /// Called before the request is handled.
    ///
    /// The middleware can modify the request before it is handled by any apps.
    async fn pre(&self, request: Request<Body>) -> Result<Request<Body>> {
        Ok(request)
    }

    /// Called after the request has been handled.
    ///
    /// The middleware can modify the response before it is sent to the client.
    async fn post(&self, response: Response<Body>) -> Result<Response<Body>> {
        Ok(response)
    }
}

/// Allows a `Middleware` to be converted into a `Box<dyn Middleware>`.
///
/// Facilitates the use of the `into` method.
impl<M: Middleware + 'static> From<M> for Box<dyn Middleware> {
    fn from(middleware: M) -> Self {
        Box::new(middleware)
    }
}
