use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io::Cursor;

#[derive(Debug, Deserialize, Serialize)]
pub enum AppError {
    SettingsNotFound(String),
    SnowRunnerProfileDirMissing(String),
    SnowRunnerNoProfile(String),
    FileCreateError(String),
    FileWriteError(String),
    AppDataDirNotFound(String),
    HomeDirNotFound(String),
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
            AppError::ProfileRestoreFailed(what) => what,
        }
    }
}

impl<'r> Responder<'r> for AppError {
    fn respond_to(self, _request: &Request) -> response::Result<'r> {
        let msg = json!(self);
        let status = match self {
            AppError::SettingsNotFound(_) => Status::InternalServerError,
            AppError::SnowRunnerProfileDirMissing(_) => Status::InternalServerError,
            AppError::SnowRunnerNoProfile(_) => Status::BadRequest,
            AppError::FileCreateError(_) => Status::InternalServerError,
            AppError::FileWriteError(_) => Status::InternalServerError,
            AppError::AppDataDirNotFound(_) => Status::InternalServerError,
            AppError::HomeDirNotFound(_) => Status::InternalServerError,
            AppError::FileReadError(_) => Status::InternalServerError,
            AppError::Unimplemented(_) => Status::Forbidden,
            AppError::MissingParameter(_) => Status::BadRequest,
            AppError::SavegameNotFound(_) => Status::BadRequest,
            AppError::ProfileRestoreFailed(_) => Status::InternalServerError,
        };
        Response::build()
            .sized_body(Cursor::new(msg.to_string()))
            .header(ContentType::JSON)
            .status(status)
            .ok()
    }
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
            AppError::AppDataDirNotFound(_) => write!(f, "AppData directory could not be found!"),
            AppError::HomeDirNotFound(_) => write!(f, "Home directory could not be found!"),
            AppError::FileReadError(_) => write!(f, "Error reading savegames from disk!"),
            AppError::Unimplemented(_) => write!(f, "Method not implemented"),
            AppError::MissingParameter(what) => write!(f, "Missing parameter {}!", what),
            AppError::SavegameNotFound(what) => write!(f, "Savegame not found: {}", what),
            AppError::ProfileRestoreFailed(what) => write!(f, "Profile restore failed: {}", what),
        }
    }
}
