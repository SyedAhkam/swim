use swim::prelude::*;
use base::BaseApp;

#[derive(Debug)]
struct AppsExample;

impl Project for AppsExample {
    fn settings(&self) -> Settings {
        Settings::builder()
            .extend_ron(relative!("settings.ron"))
            .build()
    }

    fn apps(&self) -> Vec<Box<dyn App>> {
        vec![
            BaseApp.into()
        ]
    }

    fn middlewares(&self) -> Vec<Box<dyn Middleware>> {
        vec![]
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    swim!(AppsExample, host = "localhost", port = 8000);
}
