//! The `swim_core` crate provides the core structures and traits for building a Swim application.

pub mod app;
pub mod error;
pub mod http;
pub mod macros;
pub mod middleware;
pub mod model;
pub mod project;
pub mod route;
pub mod settings;
pub mod view;

use std::{
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
};

use routerify::{Router, RouterService};

// Re-exports
pub use crate::{
    app::{App, AppConfig},
    error::Error,
    http::{Body, Request, Response, StatusCode},
    middleware::Middleware,
    model::Model,
    project::Project,
    route::Route,
    settings::{CoreSettings, DatabaseSettings, Settings},
    view::View,
};

/// Convinience Result type for the `swim` crate.
///
/// This is just a type alias for `std::result::Result<T, Error>`. Where `T` is the type of the value that is returned on success and `Error` is the swim error type.
pub type Result<T> = std::result::Result<T, Error>;

/// The `Swim` struct is the main entry point for a Swim application.
///
/// It is created using the `swim!` macro, or by using the builder (or in this case; `with`) pattern.
#[derive(Debug)]
pub struct Swim {
    project: Box<dyn Project>,
    host: String,
    port: u16,
}

impl Swim {
    /// Creates a new `Swim` instance using the given `Project`.
    pub fn with(project: Box<dyn Project>) -> Self {
        Self {
            project,
            host: "127.0.0.1".to_string(),
            port: 8000,
        }
    }

    /// Mutates the `host` field of the `Swim` instance. This is the hostname or IP address that the server will listen on.
    ///
    /// The default value is `127.0.0.1`.
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// Mutates the `port` field of the `Swim` instance. This is the port that the server will listen on.
    ///
    /// The default value is `8000`.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Starts the server and runs the application.
    ///
    /// This method is `async`, and will block until the server is stopped.
    pub async fn swim(self) {
        // Parse the host and port
        let ip_address = match self.host {
            host if host == "localhost" => Ipv4Addr::LOCALHOST,
            host => Ipv4Addr::from_str(&host).expect("Invalid IP address"),
        };
        let address = SocketAddr::from((ip_address, self.port));

        let _settings = self.project.settings();
        let apps = self.project.apps();
        let _middlewares = self.project.middlewares();

        // Returns a router
        let get_router = || {
            let mut builder = Router::<Body, Error>::builder();

            for app in apps.iter() {
                let mut scoped_router_builder = Router::builder();

                for route in app.routes() {
                    let view_get = route.view.clone();
                    let view_post = route.view.clone();
                    let view_put = route.view.clone();
                    let view_patch = route.view.clone();
                    let view_delete = route.view.clone();

                    // Add binding for GET method
                    scoped_router_builder = scoped_router_builder.get(&route.path, move |req| {
                        let view = view_get.clone();
                        async move { view.get(req).await }
                    });

                    // Add binding for POST method
                    scoped_router_builder = scoped_router_builder.post(&route.path, move |req| {
                        let view = view_post.clone();
                        async move { view.post(req).await }
                    });

                    // Add binding for PUT method
                    scoped_router_builder = scoped_router_builder.put(&route.path, move |req| {
                        let view = view_put.clone();
                        async move { view.put(req).await }
                    });

                    // Add binding for PATCH method
                    scoped_router_builder = scoped_router_builder.patch(&route.path, move |req| {
                        let view = view_patch.clone();
                        async move { view.patch(req).await }
                    });

                    // Add binding for DELETE method
                    scoped_router_builder = scoped_router_builder.delete(&route.path, move |req| {
                        let view = view_delete.clone();
                        async move { view.delete(req).await }
                    });
                }

                builder = builder.scope(app.mount(), scoped_router_builder.build().unwrap());
            }

            builder.build().unwrap()
        };

        // Make a service to handle each connection.
        let service = RouterService::new(get_router()).unwrap();

        if let Err(e) = hyper::server::Server::bind(&address).serve(service).await {
            eprintln!("Server error: {}", e);
        }
    }
}
