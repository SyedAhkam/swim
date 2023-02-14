use crate::{app::App, middleware::Middleware, settings::Settings};

/// The `Project` trait is implemented by the main application struct.
///
/// It is used to configure the application, and to provide the apps and middlewares.
///
/// Default implementations are provided for the `apps` and `middlewares` methods. You can override these methods to provide custom implementations.
///
/// Example:
///
/// ```no_run
/// # use swim_core::{Project, Settings, App, Middleware};
/// # macro_rules! relative { ($path:expr) => { ($path) } } // mock macro
///
/// #[derive(Debug)]
/// struct CoolProject;
///
/// impl Project for CoolProject {
///     fn settings(&self) -> Settings {
///         Settings::builder()
///             .extend_ron(relative!("settings.ron"))
///             .build()
///             .unwrap()
///     }
///
///     fn apps(&self) -> Vec<Box<dyn App>> {
///         vec![]
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
