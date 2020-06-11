use serde::{Deserialize, Serialize};

use std::path::PathBuf;
use std::fs::{read_dir, DirEntry, File, metadata};
use std::time::SystemTime;

use crate::error::AppError;

const DATA_PATH: &'static str = "SpinTires MudRunner\\UserSaves";
const PROFILE_PATH: &'static str = "mud-saver\\mudrunner";

#[derive(Debug, Deserialize, Serialize)]
pub struct MudrunnerSave {
    user_name: String,
    timestamp: SystemTime,
    original_name: String,
}

impl MudrunnerSave {
// function to get a vector of the mudrunner savegames' titles/user names in our app's storage
    pub fn get_archived_mudrunner_saves() -> Result<Vec<MudrunnerSave> , AppError> {
        Err(AppError::SettingsNotFound(String::from("")))
    }

    // function to get a vector of the mudrunner savegames' file names in Mudrunner's storage
    pub fn get_available_mudrunner_saves() -> Result<Vec<MudrunnerSave> , AppError> {
        let path = get_mudrunner_data_dir()?;
        let dir_listing = match read_dir(&path) {
            Ok(d) => d,
            Err(_) => return Err(AppError::MudrunnerProfileDirMissing),
        };

        let mut savegamevec :Vec<MudrunnerSave> = Vec::new();
        for entry in dir_listing {
            let mut savegame = MudrunnerSave {
                user_name: String::from(""),
                original_name: String::from(""),
                timestamp: SystemTime::now(),
            };

            match entry {
                Ok(e) => {
                    if let Some(filename) = e.file_name().to_str() {
                        savegame.original_name = String::from(filename);
                    } else {
                        continue;
                    }

                    if let Ok(time) = e.metadata().unwrap().modified() {
                        savegame.timestamp = time;
                    } else {
                        continue;
                    }
                }
                Err(err) => {
                    dbg!(err);
                    continue;
                }
            }

            savegamevec.push(savegame);
        }

        Ok(savegamevec)
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

impl Clone for MudrunnerSave {
    fn clone(&self) -> MudrunnerSave {
        let new_save = MudrunnerSave {
            user_name: self.user_name.clone(),
            timestamp: self.timestamp,
            original_name: self.original_name.clone()
        };

        return  new_save;
    }
}

fn get_mudrunner_data_dir () -> Result<PathBuf, AppError> {
    let mut path = match dirs::config_dir() {
        Some(d) => d,
        None => return Err(AppError::HomeDirNotFound(String::from("")))
    };
    path.push(DATA_PATH);
    Ok(path)
}

fn get_mudrunner_profile_dir() -> Result<PathBuf, AppError> {
    let mut path = match dirs::document_dir() {
        Some(d) => d,
        None => return Err(AppError::HomeDirNotFound(String::from(""))),
    };
    dbg!(&path);
    path.push(PROFILE_PATH);
    Ok(path)
}
