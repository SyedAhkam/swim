use swim::prelude::*;

#[derive(Debug)]
pub struct Logger;

#[async_trait::async_trait]
impl Middleware for Logger {
    async fn pre(&self, request: Request<Body>) -> Result<Request<Body>> {
        println!("New request: {:?}", request.uri());

        Ok(request)
    }

    async fn post(&self, response: Response<Body>) -> Result<Response<Body>> {
        println!("Response: {:?}", response.status());

        Ok(response)
    }
}
