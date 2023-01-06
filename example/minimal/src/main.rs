use swim::prelude::*;

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
        vec![]
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    swim!(Minimal, host = "127.0.0.1", port = 8000);

    // Or alternatively, using the builder pattern:
    // Swim::with(Box::new(Minimal))
    //     .host("127.0.0.1")
    //     .port(8000)
    //     .swim()
    //     .await;
}
