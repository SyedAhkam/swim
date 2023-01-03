#![doc = include_str!(concat!(env!("CARGO_WORKSPACE_DIR"), "/README.md"))]

/// Prelude for the `swim` crate.
pub mod prelude {
    pub use swim_core::{swim, App, Middleware, Project, Settings, Swim};
}

// Re-exports
pub use swim_core::{swim, App, Middleware, Project, Settings, Swim};
