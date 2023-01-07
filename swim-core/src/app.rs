pub mod config;

// Re-exports
pub use config::AppConfig;

use crate::{model::Model, route::Route};

/// The `App` trait is implemented to divide a project into modular units referred to as `apps`.
///
/// An `App` is a collection of `AppConfig`, `Models` and `Routes` that are mounted at a specific path.
pub trait App: std::fmt::Debug {
    /// Returns the mount path for this app.
    fn mount(&self) -> &'static str;

    /// Returns the config for this app.
    fn config(&self) -> config::AppConfig;

    /// Returns the models for this app.
    fn models(&self) -> Vec<Box<dyn Model>> {
        vec![]
    }

    /// Returns the routes for this app.
    ///
    /// At runtime, every request is matched against the routes of every app. The first route that matches the request is used to handle the request.
    fn routes(&self) -> Vec<Route> {
        vec![]
    }
}

/// Allows an `App` to be converted into a `Box<dyn App>`.
///
/// Facilitates the use of the `into` method.
impl<A: App + 'static> From<A> for Box<dyn App> {
    fn from(app: A) -> Self {
        Box::new(app)
    }
}
