/// Swim error type.
///
/// This type is used to represent errors that occur while running a Swim application.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Hyper error: {0}")]
    Hyper(#[from] hyper::Error),
}
