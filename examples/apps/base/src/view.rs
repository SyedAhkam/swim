use swim::prelude::*;

#[derive(Debug)]
pub struct HelloView;

impl View for HelloView {
    fn get(&self, request: Request<Body>) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Hello, World!"))
            .unwrap()
    }
}
