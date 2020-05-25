use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Deserialize, Serialize)]
pub enum AppError {
    SettingsNotFound(String),
    SnowRunnerProfileDirMissing(String),
    SnowRunnerNoProfile(String),
    FileCreateError(String),
    FileWriteError(String),
    AppDataDirNotFound,
    HomeDirNotFound,
    FileReadError,
    Unimplemented,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AppError::SettingsNotFound(what) => write!(f, "Settings file not found: {}", what),
            AppError::SnowRunnerProfileDirMissing(what) => {
                write!(f, "SnowRunner profile directory not found: {}", what)
            }
            AppError::SnowRunnerNoProfile(what) => {
                write!(f, "SnowRunner profile not found: {}", what)
            }
            AppError::FileCreateError(what) => write!(f, "Error creating file: {}", what),
            AppError::FileWriteError(what) => write!(f, "Error writing to file: {}", what),
            AppError::AppDataDirNotFound => write!(f, "AppData directory could not be found!"),
            AppError::HomeDirNotFound => write!(f, "Home directory could not be found!"),
            AppError::FileReadError => write!(f, "Error reading savegames from disk!"),
            AppError::Unimplemented => write!(f, "Method not implemented"),
        }
    }
}
