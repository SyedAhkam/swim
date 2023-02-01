#![doc = include_str!("../../README.md")]
#![forbid(unsafe_code)]

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
