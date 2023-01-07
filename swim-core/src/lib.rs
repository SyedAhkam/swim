//! The `swim_core` crate provides the core structures and traits for building a Swim application.

pub mod app;
pub mod http;
pub mod macros;
pub mod middleware;
pub mod model;
pub mod project;
pub mod route;
pub mod settings;

use std::{
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
};

// Re-exports
pub use crate::{
    app::{App, AppConfig},
    middleware::Middleware,
    model::Model,
    project::Project,
    route::Route,
    settings::{CoreSettings, DatabaseSettings, Settings},
};

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
        let _settings = self.project.settings();
        let _apps = self.project.apps();
        let _middlewares = self.project.middlewares();

        // Parse the host and port
        let ip_address = match self.host {
            host if host == "localhost" => Ipv4Addr::LOCALHOST,
            host => Ipv4Addr::from_str(&host).expect("Invalid IP address"),
        };
        let address = SocketAddr::from((ip_address, self.port));

        // Bind the server
        let make_svc = hyper::service::make_service_fn(move |_| async move {
            Ok::<_, hyper::Error>(hyper::service::service_fn(move |_req| async move {
                let mut response = hyper::Response::new(hyper::Body::empty());

                // Apply server-specific headers to the response.
                http::apply_server_specific_headers(&mut response);

                Ok::<_, hyper::Error>(response)
            }))
        });

        if let Err(e) = hyper::server::Server::bind(&address).serve(make_svc).await {
            eprintln!("Server error: {}", e);
        }
    }
}
