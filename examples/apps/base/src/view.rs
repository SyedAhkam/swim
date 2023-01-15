use swim::prelude::*;

#[derive(Debug)]
pub struct HelloView;

// Async traits are not yet stable, so we have to use the async_trait crate.
#[async_trait::async_trait]
impl View for HelloView {
    async fn get(&self, request: Request<Body>) -> Result<Response<Body>> {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Say hello to Swim!"))
            .unwrap())
    }

    async fn post(&self, request: Request<Body>) -> Result<Response<Body>> {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("It's a post request!"))
            .unwrap())
    }
}
