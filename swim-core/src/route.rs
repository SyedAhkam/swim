use std::sync::Arc;

use crate::view::View;

/// A route is a mapping between a path and a view.
///
/// When a request is made to a path, the associated view is called to handle the request.
#[derive(Debug)]
pub struct Route {
    pub path: String,
    pub view: Arc<dyn View>,
}

impl Route {
    /// Creates a new route.
    pub fn new(path: &str, view: impl View) -> Self {
        Self {
            path: path.to_string(),
            view: Arc::new(view),
        }
    }
}
