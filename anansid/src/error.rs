use std::str::Utf8Error;

use crate::runner::ExitStatus;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Utf8(#[from] Utf8Error),
    #[error("No process was supplied to be executed")]
    NoProcessToRun,
    #[error("Process failed with status `{0}`")]
    ProcessFailed(ExitStatus),
}
