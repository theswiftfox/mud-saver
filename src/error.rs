use actix_web::{error, http::header, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Deserialize, Serialize)]
pub enum AppError {
    SettingsNotFound(String),
    HomeDirNotFound(String),
    MudrunnerProfileDirMissing(String),
    MudrunnerArchiveDirMissing(String),
    SnowRunnerProfileDirMissing(String),
    SnowRunnerNoProfile(String),
    FileCreateError(String),
    FileWriteError(String),
    AppDataDirNotFound(String),
    FileReadError(String),
    Unimplemented(String),
    MissingParameter(String),
    SavegameNotFound(String),
    ProfileRestoreFailed(String),
}

impl Error for AppError {
    fn description(&self) -> &str {
        match self {
            AppError::SettingsNotFound(what) => what,
            AppError::SnowRunnerProfileDirMissing(what) => what,
            AppError::SnowRunnerNoProfile(what) => what,
            AppError::FileCreateError(what) => what,
            AppError::FileReadError(what) => what,
            AppError::FileWriteError(what) => what,
            AppError::AppDataDirNotFound(what) => what,
            AppError::HomeDirNotFound(what) => what,
            AppError::Unimplemented(what) => what,
            AppError::MissingParameter(what) => what,
            AppError::SavegameNotFound(what) => what,
            AppError::MudrunnerProfileDirMissing(what) => what,
            AppError::MudrunnerArchiveDirMissing(what) => what,
            AppError::ProfileRestoreFailed(what) => what,
        }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::SettingsNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SnowRunnerProfileDirMissing(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SnowRunnerNoProfile(_) => StatusCode::BAD_REQUEST,
            AppError::FileCreateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::FileWriteError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AppDataDirNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::HomeDirNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::FileReadError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unimplemented(_) => StatusCode::FORBIDDEN,
            AppError::SavegameNotFound(_) => StatusCode::BAD_REQUEST,
            AppError::MissingParameter(_) => StatusCode::BAD_REQUEST,
            AppError::MudrunnerProfileDirMissing(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::MudrunnerArchiveDirMissing(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ProfileRestoreFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AppError::SettingsNotFound(what) => write!(f, "Settings file not found: {}", what),
            AppError::HomeDirNotFound(what) => {
                write!(f, "Home directory could not be found: {}", what)
            }
            AppError::MudrunnerProfileDirMissing(_) => {
                write!(f, "Directory of Mudrunner savegames missing or corrupted")
            }
            AppError::MudrunnerArchiveDirMissing(_) => write!(
                f,
                "Directory of archived Mudrunner savegames missing or corrupted"
            ),
            AppError::SnowRunnerProfileDirMissing(what) => {
                write!(f, "SnowRunner profile directory not found: {}", what)
            }
            AppError::SnowRunnerNoProfile(what) => {
                write!(f, "SnowRunner profile not found: {}", what)
            }
            AppError::FileCreateError(what) => write!(f, "Error creating file: {}", what),
            AppError::FileWriteError(what) => write!(f, "Error writing to file: {}", what),
            AppError::AppDataDirNotFound(_) => write!(f, "AppData directory could not be found!"),
            AppError::FileReadError(_) => write!(f, "Error reading savegames from disk!"),
            AppError::Unimplemented(_) => write!(f, "Method not implemented"),
            AppError::MissingParameter(what) => write!(f, "Missing parameter {}!", what),
            AppError::SavegameNotFound(what) => write!(f, "Savegame not found: {}", what),
            AppError::ProfileRestoreFailed(what) => write!(f, "Profile restore failed: {}", what),
        }
    }
}
