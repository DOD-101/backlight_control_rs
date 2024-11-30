//! Custom [Error] for backlight_control_rs
//!
//! The main use of this error is to be able to have one error for both the [io::Error]s and the
//! case in which the user doesn't have a backlight.
use std::{fmt, io};

use crate::BRIGHTNESS_BASE_PATH;

#[derive(Debug)]
pub enum Error {
    /// A wrapper around an [io::Error]
    IoError(io::Error),
    /// Error emitted when there is no backlight
    FailedToGetFirstEntry,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "{}", e),
            Error::FailedToGetFirstEntry => write!(
                f,
                "Failed to get the first dir in {} this most probably means you have no backlight.",
                BRIGHTNESS_BASE_PATH
            ),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
