/// Generates a crate-relative version of a path.
///
/// It is capable of generating system-specific paths. Uses `/` as a path separator on Unix-like systems and `\` on Windows.
///
/// # Examples
///
/// ```no_run
/// use swim_util::relative;
///
/// let path = relative!("settings.ron");
///
/// assert_eq!(path, "/home/user/dev/mycoolproject/settings.ron");
/// ```
#[macro_export]
macro_rules! relative {
    ($path:expr) => {
        if cfg!(target_os = "windows") {
            concat!(env!("CARGO_MANIFEST_DIR"), "\\", $path)
        } else {
            concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)
        }
    };
}
