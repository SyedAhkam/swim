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

#[derive(Debug)]
pub struct GreetingView;

#[async_trait::async_trait]
impl View for GreetingView {
    async fn get(&self, request: Request<Body>) -> Result<Response<Body>> {
        let name = request.param("name").unwrap(); // unwrap is safe because the route wouldn't have matched if the param wasn't there

        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(format!("Hello, {}!", name)))
            .unwrap())
    }
}