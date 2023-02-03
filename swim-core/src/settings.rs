use std::{fs::File, io::Read, path::Path};

use serde::Deserialize;

/// The `CoreSettings` struct holds basic, yet essential information about the application.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct CoreSettings {
    pub name: String,
    pub secret_key: String,
    pub language_code: String,
    pub timezone: String,
}

impl Default for CoreSettings {
    fn default() -> Self {
        Self {
            name: "DEFAULT_NAME".to_string(),
            secret_key: "DEFAULT_SECRET_KEY".to_string(),
            language_code: "en-us".to_string(),
            timezone: "UTC".to_string(),
        }
    }
}

/// The `DatabaseSettings` struct holds information about the database connection.
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,

    #[serde(rename = "type")]
    pub type_: String,
}

/// The `Settings` struct is used to configure the application.
///
/// It is created using the [SettingsBuilder] struct, which is returned by the `Settings::builder()` method.
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub core: CoreSettings,
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn builder() -> SettingsBuilder {
        SettingsBuilder::new()
    }
}

/// The `SettingsBuilder` struct is used to build the `Settings` struct.
///
/// It is created using the [Settings::builder()] method. Or alternatively, by loading the settings from a [ron](https://github.com/ron-rs/ron) file.
///
/// # Examples
///
/// ```rust
/// # use swim_core::settings::{Settings, CoreSettings, DatabaseSettings};
///
/// let settings = Settings::builder()
///     .core(CoreSettings {
///         name: "My App".to_string(),
///         secret_key: "My Secret Key".to_string(),
///         ..Default::default()
///     })
///     .database(DatabaseSettings {
///         url: "sqlite://my_database.db".to_string(),
///         type_: "sqlite".to_string(),
///     })
///     .build();
/// ```
///
/// ```no_run
/// use swim_core::settings::Settings;
///
/// let settings = Settings::builder()
///    .extend_ron(concat!(env!("CARGO_MANIFEST_DIR"), "/settings.ron"))
///    .build();
/// ```
pub struct SettingsBuilder {
    core: Option<CoreSettings>,
    database: Option<DatabaseSettings>,
}

impl SettingsBuilder {
    /// Creates a new `SettingsBuilder` instance.
    pub fn new() -> Self {
        Self {
            core: None,
            database: None,
        }
    }

    /// Mutates the `core` field of the `SettingsBuilder` instance.
    pub fn core(mut self, core: CoreSettings) -> Self {
        self.core = Some(core);
        self
    }

    /// Mutates the `database` field of the `SettingsBuilder` instance.
    pub fn database(mut self, database: DatabaseSettings) -> Self {
        self.database = Some(database);
        self
    }

    /// Loads the settings from a `.ron` file.
    pub fn extend_ron<P: AsRef<Path> + std::fmt::Debug>(mut self, path: P) -> Self {
        // Read the file
        let mut file = File::open(path).expect("Failed to open settings file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read settings file");

        // Parse the file
        let settings: Settings = ron::from_str(&contents).expect("Failed to parse settings file");

        // Load the settings
        self.core = Some(settings.core);
        self.database = Some(settings.database);

        self
    }

    /// Builds the `Settings` instance.
    pub fn build(self) -> Settings {
        let core = self.core.expect("Core settings are required");
        let database = self.database.expect("Database settings are required");

        if core.secret_key == "DEFAULT_SECRET_KEY" {
            panic!("You must set a secret key");
        }

        Settings { core, database }
    }
}
