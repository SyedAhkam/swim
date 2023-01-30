use crate::{
    http::{Body, Request, Response, StatusCode},
    Result,
};

macro_rules! blank_status_response {
    ($status:expr) => {
        Ok(Response::builder()
            .status($status)
            .body(Body::empty())
            .unwrap())
    };
}

/// Implement this trait to create a view.
///
/// A view is a collection of methods that are called when a request is made to a specific route.
///
/// The methods are called based on the HTTP method of the request. For example, if a request is made to a route with a `GET` method, the `get` method will be called.
///
/// If a method is not implemented, a `405 Method Not Allowed` response will be returned.
///
/// # Example
///
/// ```
/// use swim::prelude::*;
///
/// #[derive(Debug)]
/// pub struct HelloView;
///
/// impl View for HelloView {
///    fn get(&self, request: Request<Body>) -> Response<Body> {
///       Response::builder()
///         .status(StatusCode::OK)
///         .body(Body::from("Hello, World!"))
///         .unwrap()
///   }
/// }
/// ```
#[allow(unused_variables)]
#[async_trait::async_trait]
pub trait View: std::fmt::Debug + Send + Sync + 'static {
    /// Called when a request is made to a route with a `GET` method.
    async fn get(&self, request: Request<Body>) -> Result<Response<Body>> {
        blank_status_response!(StatusCode::METHOD_NOT_ALLOWED)
    }

    /// Called when a request is made to a route with a `POST` method.
    async fn post(&self, request: Request<Body>) -> Result<Response<Body>> {
        blank_status_response!(StatusCode::METHOD_NOT_ALLOWED)
    }

    /// Called when a request is made to a route with a `PUT` method.
    async fn put(&self, request: Request<Body>) -> Result<Response<Body>> {
        blank_status_response!(StatusCode::METHOD_NOT_ALLOWED)
    }

    /// Called when a request is made to a route with a `PATCH` method.
    async fn patch(&self, request: Request<Body>) -> Result<Response<Body>> {
        blank_status_response!(StatusCode::METHOD_NOT_ALLOWED)
    }

    /// Called when a request is made to a route with a `DELETE` method.
    async fn delete(&self, request: Request<Body>) -> Result<Response<Body>> {
        blank_status_response!(StatusCode::METHOD_NOT_ALLOWED)
    }
}
