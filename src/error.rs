use thiserror::Error;

use crate::message::MessageError;

/// Fatal errors that stop the server entirely.
#[derive(Debug, Error)]
pub enum ServerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Errors that occur while handling a single request; the server logs these
/// and keeps running.
#[derive(Debug, Error)]
pub enum RequestError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid DNS message: {0}")]
    Message(#[from] MessageError),
}
