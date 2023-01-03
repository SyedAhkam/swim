/// A macro to simplify the creation of a new `Swim` instance.
///
/// # Examples
///
/// ```
/// #[tokio::main]
/// async fn main() {
///    swim!(Minimal, host="0.0.0.0", port=8000);
/// }
#[macro_export]
macro_rules! swim {
    ($project:ident $(,host=$host:expr)? $(,port=$port:expr)?) => {{
        Swim::with(Box::new($project))
            $(.host($host))?
            $(.port($port))?
            .swim()
            .await
    }};
}
