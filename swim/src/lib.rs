// Use cargo rdme to generate the README.md file from this doc comment.
//! An opinionated batteries-included approach to a rust web framework.
//!
//! The idea is to take the best parts of the rust ecosystem and combine them into a framework that is easy to use and provides a good developer experience.
//!
//! # Features
//!
//! - Go blazingly fast with [hyper](https://github.com/hyperium/hyper) and [tokio](https://github.com/tokio-rs/tokio)
//! - Powerful routing with [routerify](https://github.com/routerify/routerify)
//! - CLI tooling with [cargo-swim](cargo-swim) (coming soon)
//! - Database support with [SeaORM](https://github.com/SeaQL/sea-orm) (planned)
//! - Templating with [Tera](https://github.com/Keats/tera) (planned)
//! - Dependency injection (planned)
//!
//! Note: This project is still in development.
//!
//! # Building a project
//!
//! You define a project by defining a struct that implements the `Project` trait. It is the highest-level abstraction in the framework. It is responsible for defining the settings, apps, and middleware for your project.
//!
//! ```rs
//! struct MyProject;
//!
//! impl Project for MyProject {
//!     fn settings(&self) -> Settings {
//!         Settings::builder()
//!             .extend_ron(relative! ("settings.ron"))
//!             .build()
//!     }
//!
//!     fn apps(&self) -> Vec<Box<dyn App>> {
//!         vec! [
//!             MyApp.into()
//!         ]
//!     }
//!
//!     fn middleware(&self) -> Vec<Box<dyn Middleware>> {
//!         vec! [
//!             MyMiddleware.into()
//!         ]
//!     }
//! }
//!
//! ```
//!
//! # Building apps
//!
//! You define an app by defining a struct that implements the `App` trait. It is responsible for defining the routes and views for your app.
//!
//! ```rs
//! struct MyApp;
//!
//! impl App for MyApp {
//!     fn mount(&self) -> &'static str {
//!         "/"
//!     }
//!
//!     fn config(&self) -> AppConfig {
//!         AppConfig::with_name("MyApp")
//!     }
//!
//!     fn models(&self) -> Vec<Box<dyn Model>> {
//!         vec! []
//!     }
//!
//!     fn routes(&self) -> Vec<Route> {
//!         vec! [
//!             Route::new("/", IndexView),
//!             Route::new("/hello", HelloView)
//!         ]
//!     }
//! }
//!
//! ```
//!
//! # Building views
//!
//! You define a view by defining a struct that implements the `View` trait. It is responsible for handling the request and returning a response. You can implement the specific HTTP methods you want to handle.
//!
//! ```rs
//! #[derive(Debug)]
//! pub struct HelloView;
//!
//! #[async_trait::async_trait]
//! impl View for HelloView {
//!     async fn get(&self, request: Request<Body>) -> Result<Response<Body>> {
//!         Ok(Response::builder()
//!             .status(StatusCode::OK)
//!             .body(Body::from("Say hello to Swim! "))
//!             .unwrap())
//!     }
//!
//!     async fn post(&self, request: Request<Body>) -> Result<Response<Body>> {
//!         Ok(Response::builder()
//!             .status(StatusCode::OK)
//!             .body(Body::from("It's a post request! "))
//!             .unwrap())
//!     }
//! }
//!
//! ```
//!
//! # Defining middlewares
//!
//! You define a middleware by defining a struct that implements the `Middleware` trait. It is responsible for handling the request and returning a response. You can implement the specific HTTP methods you want to handle.
//!
//! ```rs
//! ##[derive(Debug)]
//! pub struct Logger;
//!
//! #[async_trait::async_trait]
//! impl Middleware for Logger {
//!     async fn pre(&self, request: Request<Body>) -> Result<Request<Body>> {
//!         println! ("New request: {:?}", request.uri());
//!
//!         Ok(request)
//!     }
//!
//!     async fn post(&self, response: Response<Body>) -> Result<Response<Body>> {
//!         println! ("Response: {:?}", response.status());
//!
//!         Ok(response)
//!     }
//! }
//! ```
//!
//! # Running the project
//!
//! You may use the elegant swim macro to run your project.
//!
//! ```rs
//! #[tokio::main(flavor = "multi_thread")]
//! async fn main() {
//!     swim! (MyProject, host = "localhost", port = 8000);
//! }
//! ```

#[forbid(unsafe_code)]

/// Prelude for the `swim` crate.
pub mod prelude {
    pub use swim_core::{
        swim, App, AppConfig, Body, CoreSettings, DatabaseSettings, Middleware, Model, Project,
        Request, Response, Result, Route, Settings, StatusCode, Swim, View,
    };

    pub use swim_util::relative;
}

// Re-exports
pub use swim_core::{
    swim, App, AppConfig, Body, CoreSettings, DatabaseSettings, Error, Middleware, Model, Project,
    Request, Response, Result, Route, Settings, StatusCode, Swim, View,
};

/// Various utilities that are helpful while building a Swim application.
pub mod util {
    pub use swim_util::relative;
}
