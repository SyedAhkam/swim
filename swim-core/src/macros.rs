/// A macro to simplify the creation of a new `Swim` instance.
///
/// `host` and `port` are optional parameters, and will default to `127.0.0.1` and `8000` respectively.
///
/// # Examples
///
/// ```no_run
/// # use swim_core::{swim, Swim, Settings, Project};
/// # #[derive(Debug)]
/// # struct Minimal;
/// # // Mock implementation of Project
/// # impl Project for Minimal { fn settings(&self) -> Settings { unimplemented!() } }
/// # macro_rules! relative { ($path:expr) => { ($path) } } // mock macro
///
/// #[tokio::main]
/// async fn main() {
///    swim!(Minimal, host="0.0.0.0", port=8000);
/// }
/// ```
#[macro_export]
macro_rules! swim {
    ($project:ident $(,host=$host:expr)? $(,port=$port:expr)?) => {{
        Swim::with(Box::new($project))
            $(.host($host))?
            $(.port($port))?
            .swim()
            .await.map_err(|err| {
                eprintln!("⚠️  Launch failed: {}", err);

                err
            })
            .unwrap();
    }};
}
