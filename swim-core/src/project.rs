use crate::{app::App, middleware::Middleware, settings::Settings};

/// The `Project` trait is implemented by the main application struct.
///
/// It is used to configure the application, and to provide the apps and middlewares.
///
/// Default implementations are provided for the `apps` and `middlewares` methods. You can override these methods to provide custom implementations.
///
/// Example:
///
/// ```rust
/// use swim::prelude::*;
///
/// #[derive(Debug)]
/// struct CoolProject;
///
/// impl Project for CoolProject {
///     fn settings(&self) -> Settings {
///         Settings::builder().extend_ron("settings.ron").build()
///     }
///
///     fn apps(&self) -> Vec<Box<dyn App>> {
///         vec![
///             AdminApp::builder().mount_at("/admin").build().into()
///         ]
///     }
///
///     fn middlewares(&self) -> Vec<Box<dyn Middleware>> {
///         vec![]
///     }
/// }
/// ```
pub trait Project: std::fmt::Debug {
    /// Returns the settings for this project.
    fn settings(&self) -> Settings;

    /// Returns the apps for this project.
    fn apps(&self) -> Vec<Box<dyn App>> {
        vec![]
    }

    /// Returns the middlewares for this project.
    fn middlewares(&self) -> Vec<Box<dyn Middleware>> {
        vec![]
    }
}
