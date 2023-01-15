use swim::prelude::*;
use view::HelloView;

pub mod view;

#[derive(Debug)]
pub struct BaseApp;

impl App for BaseApp {
    fn mount(&self) -> &'static str {
        "/"
    }

    fn config(&self) -> AppConfig {
        AppConfig::with_name("BaseApp")
    }

    fn models(&self) -> Vec<Box<dyn Model>> {
        vec![]
    }

    fn routes(&self) -> Vec<Route> {
        vec![Route::new("/hello", HelloView)]
    }
}
