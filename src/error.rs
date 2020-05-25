use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Deserialize, Serialize)]
pub enum AppError {
    SettingsNotFound(String),
    SnowRunnerProfileDirMissing(String),
    SnowRunnerNoProfile(String),
    HomeDirNotFound,
    FileReadError,
    Unimplemented,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AppError::SettingsNotFound(what) => write!(f, "Settings file not found: {}", what),
            AppError::SnowRunnerProfileDirMissing(what) => write!(f, "SnowRunner profile directory not found: {}", what),
            AppError::SnowRunnerNoProfile(what) => write!(f, "SnowRunner profile not found: {}", what),
            AppError::HomeDirNotFound => write!(f, "Home directory could not be found!"),
            AppError::FileReadError => write!(f, "Error reading savegames from disk!"),
            AppError::Unimplemented => write!(f, "Method not implemented"),
        }
    }
}
