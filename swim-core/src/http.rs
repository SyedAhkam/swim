const SERVER_HEADER: &str = concat!("swim/", env!("CARGO_PKG_VERSION"));

/// A type alias for a `hyper` request body.
pub type Body = hyper::Body;

/// A type alias for a `hyper` status code.
pub type StatusCode = hyper::StatusCode;

/// A type alias for a `hyper` request.
pub type Request<T> = hyper::Request<T>;

/// A type alias for a `hyper` response.
pub type Response<T> = hyper::Response<T>;

pub fn apply_server_specific_headers<T>(response: &mut hyper::Response<T>) {
    response
        .headers_mut()
        .insert("Server", SERVER_HEADER.parse().unwrap());
}
