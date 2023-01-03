use swim::prelude::*;
// use swim_contrib::admin::AdminApp;

#[derive(Debug)]
struct Minimal;

impl Project for Minimal {
    fn settings(&self) -> Settings {
        // Settings::builder().extend_ron("settings.ron").build()
        Settings::builder().build()
    }

    fn apps(&self) -> Vec<Box<dyn App>> {
        vec![
            // AdminApp::builder().mount_at("/admin").build().into()
        ]
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
