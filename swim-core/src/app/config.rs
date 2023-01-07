/// `AppConfig` stores the configuration for each `App`.
///
/// For now, it only stores the name of the app.
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub name: String,
}

impl AppConfig {
    pub fn with_name(name: &str) -> AppConfig {
        AppConfig {
            name: name.to_string(),
        }
    }
}
