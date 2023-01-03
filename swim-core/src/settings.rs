pub struct SettingsBuilder;

impl SettingsBuilder {
    pub fn build(self) -> Settings {
        Settings
    }
}

#[derive(Debug)]
pub struct Settings;

impl Settings {
    pub fn builder() -> SettingsBuilder {
        SettingsBuilder
    }
}
