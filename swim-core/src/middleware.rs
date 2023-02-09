use hyper::body::HttpBody;

use crate::{request_info, response_info, Body, Request, Response, Result};

/// The `Middleware` trait is implemented by middleware structs.
///
/// It is used to intercept requests and responses. And optionally to modify them.
///
/// Example:
///
/// ```
/// # use swim_core::{Middleware, Request, Response, Result, Body};
///
/// #[derive(Debug)]
/// struct Logger;
///
/// #[async_trait::async_trait]
/// impl Middleware for Logger {
///     async fn pre(&self, request: Request<Body>) -> Result<Request<Body>> {
///         println!("New request: {:?}", request.uri());
///
///         Ok(request)
///     }
///
///     async fn post(&self, response: Response<Body>) -> Result<Response<Body>> {
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

/// A built-in middleware that logs requests and responses.
///
/// This middleware is enabled by default.
#[derive(Debug)]
pub struct Logger;

#[async_trait::async_trait]
impl Middleware for Logger {
    async fn pre(&self, request: Request<Body>) -> Result<Request<Body>> {
        request_info!(request);

        Ok(request)
    }

    async fn post(&self, response: Response<Body>) -> Result<Response<Body>> {
        response_info!(response);

        Ok(response)
    }
}
