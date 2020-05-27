use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Deserialize, Serialize)]
pub enum AppError {
    SettingsNotFound(String),
    SRnoProfileFound(String),
    HomeDirNotFound(String),
    MudrunnerProfileDirMissing,
    MudrunnerArchiveDirMissing,
    Unimplemented,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AppError::SettingsNotFound(what) => write!(f, "Settings file not found: {}", what),
            AppError::SRnoProfileFound(what) => write!(f, "SnowRunner profile not found: {}", what),
            AppError::HomeDirNotFound(what) => {
                write!(f, "Home directory could not be found: {}", what)
            }
            AppError::Unimplemented => write!(f, "Method not implemented"),
            AppError::MudrunnerProfileDirMissing => write!(f, "Directory of Mudrunner savegames missing or corrupted"),
            AppError::MudrunnerArchiveDirMissing => write!(f, "Directory of archived Mudrunner savegames missing or corrupted")
        }
    }
}
