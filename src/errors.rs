use macroquad::Error;
use std::{any::Any, path::PathBuf};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, GameError>;
pub type Nresult = Result<()>;

#[allow(unused)]
#[derive(Error, Debug)]
pub enum GameError {
    #[error("Could not find asset: {0}")]
    NotFoundError(PathBuf),
    #[error("Invalid State in Object {0:?}")]
    InvalidState(Box<dyn Any>),
    #[error("Timeout occured: {0}")]
    Timeout(String),
    #[error("Failed to load asset: {0}")]
    AssetLoadFailure(String),
    #[error("Unsupported Operation: {0}")]
    UnsupportedOperation(String),
    #[error("Unspecified I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Illegal Integer: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Unexpected Error(!!!): {0}")]
    Unexpected(String),
    #[error("Received Illegal Arguments!!! This should not happen!!! Message: {0}")]
    IllegalArgument(String),
    #[error("Macroquad Error: {0}")]
    MQError(#[from] Error),
    #[error("Miscellaneous Error: {0:?}")]
    Misc(#[from] Box<dyn std::error::Error>),
}
