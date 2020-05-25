use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use crate::error::AppError;

const PROFILE_PATH: &'static str = "\\SpinTires MudRunner\\UserSaves";

#[derive(Debug, Deserialize, Serialize)]
pub struct MudrunnerSave {
    user_name: String,
    file_hash: u64,
    original_name: String,
}

impl MudrunnerSave {
// unction to get a vector of the mudrunner savegames' titles/user names in our app's storage
    pub fn get_archived_mudrunner_saves() -> Result<Vec<MudrunnerSave> , AppError> {
        Err(AppError::SettingsNotFound(String::from("")))
    }

    // function to get a vector of the mudrunner savegames' file names in Mudrunner's storage
    pub fn get_available_mudrunner_saves() -> Result<Vec<MudrunnerSave> , AppError> {
        Err(AppError::SettingsNotFound(String::from("")))
    }

    // function to archive a specific savegame to our app's storage
    pub fn archive_savegame(& self, savegame: &MudrunnerSave) -> Result<(), AppError> {
        Err(AppError::SettingsNotFound(String::from("")))
    }

    // function to install a specific savegame (overwriting the existing one)
    pub fn install_savegame(& self, savegame: &MudrunnerSave) -> Result<(), AppError> {
        Err(AppError::SettingsNotFound(String::from("")))
    }
}

fn get_mudrunner_data_dir () -> Result<PathBuf, AppError> {
    let mut path = match dirs::config_dir() {
        Some(d) => d,
        None => return Err(AppError::HomeDirNotFound(String::from("")))
    };
    path.push(PROFILE_PATH);
    Ok(path)
}