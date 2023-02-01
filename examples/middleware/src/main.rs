use swim::prelude::*;

pub mod middlewares;

#[derive(Debug)]
struct Minimal;

impl Project for Minimal {
    fn settings(&self) -> Settings {
        Settings::builder()
            .extend_ron(relative!("settings.ron"))
            .build()
    }

    fn apps(&self) -> Vec<Box<dyn App>> {
        vec![]
    }

    fn middlewares(&self) -> Vec<Box<dyn Middleware>> {
        vec![
            middlewares::Logger.into(),
        ]
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    swim!(Minimal, host = "localhost", port = 8000);
}
