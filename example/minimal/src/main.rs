use swim::prelude::*;
use swim_contrib::admin::AdminApp;

#[derive(Swim)]
struct Minimal;

impl Project for Minimal {
    fn settings(&self) -> SwimSettings {
        SwimSettings::builder().extend_ron("settings.ron").build()
    }

    fn apps(&self) -> Vec<Box<dyn App>> {
        vec![
            AdminApp::builder().mount_at("/admin").build().into()
        ]
    }

    fn middlewares(&self) -> Vec<Box<dyn Middleware>> {
        vec![]
    }
}

#[tokio::main]
async fn main() {
    Minimal::new()
        .host("0.0.0.0")
        .port(8000)
        .swim()
        .await;
}
