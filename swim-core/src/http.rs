use hyper::Response;

const SERVER_HEADER: &str = concat!("swim/", env!("CARGO_PKG_VERSION"));

pub fn apply_server_specific_headers<T>(response: &mut Response<T>) {
    response
        .headers_mut()
        .insert("Server", SERVER_HEADER.parse().unwrap());
}
