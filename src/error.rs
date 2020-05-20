use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Deserialize, Serialize)]
pub enum AppError {
    SettingsNotFound(String),
    SRnoProfileFound(String),
    HomeDirNotFound(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AppError::SettingsNotFound(what) => write!(f, "Settings file not found: {}", what),
            AppError::SRnoProfileFound(what) => write!(f, "SnowRunner profile not found: {}", what),
            AppError::HomeDirNotFound(what) => {
                write!(f, "Home directory could not be found: {}", what)
            }
        }
    }
}
