//! The `swim_core` crate provides the core structures and traits for building a Swim application.

pub mod app;
pub mod macros;
pub mod middleware;
pub mod project;
pub mod settings;

// Re-exports
pub use crate::{app::App, middleware::Middleware, project::Project, settings::Settings};

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
        println!("{:#?}", self);

        let settings = self.project.settings();
        let apps = self.project.apps();
        let middlewares = self.project.middlewares();

        loop {}
    }
}
