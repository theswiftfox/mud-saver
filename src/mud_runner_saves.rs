use serde::{Deserialize, Serialize};

use chrono::{offset::Utc, DateTime};
use std::fs::{copy, metadata, read_dir, DirEntry, File, remove_file};
use std::io::{BufReader, Error};
use std::path::PathBuf;

use crate::error::AppError;

const DATA_PATH: &'static str = "SpinTires MudRunner\\UserSaves";
const PROFILE_PATH: &'static str = "mr-data";

#[derive(Debug, Deserialize, Serialize)]
pub struct MudrunnerSave {
    user_name: String,
    timestamp: DateTime<Utc>,
    original_name: String,
    internal_filename: Option<PathBuf>,
}

impl MudrunnerSave {
    // function to get a vector of the mudrunner savegames' titles/user names in our app's storage
    pub fn get_archived_mudrunner_saves() -> Result<Vec<MudrunnerSave>, AppError> {
        let mut path = get_mudrunner_profile_dir()?;
        path.push("MudrunnerMetadata.json");

        let archived_saves = match File::open(&path) {
            Ok(f) => {
                let reader = BufReader::new(f);
                match serde_json::from_reader::<BufReader<File>, Vec<MudrunnerSave>>(reader) {
                    Ok(saves) => {
                        saves
                    }
                    Err(e) => {
                        dbg!(&e);
                        return Err(AppError::FileReadError(String::from(
                            "Error reading \"MudrunnerMetadata.json\"",
                        )));
                    }
                }
            }
            Err(_) => Vec::<MudrunnerSave>::new()
        };

        Ok(archived_saves)
    }

    // function to get a vector of the mudrunner savegames' file names in Mudrunner's storage
    pub fn get_available_mudrunner_saves() -> Result<Vec<MudrunnerSave>, AppError> {
        let path = get_mudrunner_data_dir()?;
        let dir_listing = match read_dir(&path) {
            Ok(d) => d,
            Err(_) => {
                return Err(AppError::MudrunnerProfileDirMissing(String::from(
                    "Mudrunner profile directory missing",
                )))
            }
        };

        let mut savegamevec: Vec<MudrunnerSave> = Vec::new();
        for entry in dir_listing {
            match entry {
                Ok(e) => {
                    let modified = match match e.metadata() {
                        Ok(meta) => match meta.modified() {
                            Ok(modified) => Some(modified),
                            Err(_) => None,
                        },
                        Err(_) => None,
                    } {
                        Some(t) => DateTime::from(t),
                        None => Utc::now(),
                    };

                    let mut savegame = MudrunnerSave {
                        user_name: String::from(""),
                        original_name: String::from(""),
                        timestamp: modified,
                        internal_filename: None
                    };

                    if let Some(filename) = e.file_name().to_str() {
                        savegame.original_name = String::from(filename);
                    } else {
                        continue;
                    }

                    savegamevec.push(savegame);
                }
                Err(err) => {
                    dbg!(err);
                    continue;
                }
            }
        }

        Ok(savegamevec)
    }

    // function to archive a specific savegame to our app's storage
    pub fn archive_savegame(user_name: &str, original_name: &str) -> Result<(), AppError> {
        let mut path = get_mudrunner_profile_dir()?;
        path.push("MudrunnerMetadata.json");
        let mut existing_saves = match File::open(&path) {
            Ok(f) => {
                let reader = BufReader::new(f);
                match serde_json::from_reader::<BufReader<File>, Vec<MudrunnerSave>>(reader) {
                    Ok(saves) => saves,
                    Err(e) => {
                        dbg!(&e);
                        return Err(AppError::FileReadError(String::from(
                            "Error reading \"MudrunnerMetadata.json\"",
                        )));
                    }
                }
            }
            Err(_) => Vec::<MudrunnerSave>::new(),
        };

        // first check if we have an element with the same "user_name", if yes we
        // take it out and later put the now one back in.
        // mudrunnersave_vec.retain(|&x| x.user_name != self.user_name);
        if let Some(existing_save) = existing_saves.iter().find(|&s| s.user_name.eq(user_name)) {
            if let Some(f) = &existing_save.internal_filename {
                remove_file(f);
            }
            existing_saves.retain(|s| !s.user_name.eq(user_name))
        }

        // Copy the actual savegame from mudrunner
        let mut from: PathBuf = match get_mudrunner_data_dir() {
            Ok(d) => d,
            Err(_) => {
                return Err(AppError::MudrunnerProfileDirMissing(String::from(
                    "Mudrunner profile directory missing",
                )));
            }
        };
        from.push(original_name);
        dbg!(&from);
        let mut to: PathBuf = match get_mudrunner_profile_dir() {
            Ok(d) => d,
            Err(_) => {
                return Err(AppError::MudrunnerProfileDirMissing(String::from(
                    "Mudrunner profile directory missing",
                )));
            }
        };

        if !to.exists() {
            match std::fs::create_dir(&to) {
                Ok(_) => (),
                Err(e) => {
                    dbg!(&to, &e);
                    return Err(AppError::FileWriteError(String::from(
                        "Unable to create data directory for backups.",
                    )));
                }
            }
        }

        to.push(uuid::Uuid::new_v4().to_string());
        dbg!(&to);
        if let Err(_) = copy(&from, &to) {
            return Err(AppError::FileWriteError(String::from("Couldn't write backup. Whats wrong with you (or us???)?")));
        };

        // Get the file's timestamp
        let modified = match match metadata(&from) {
            Ok(meta) => match meta.modified() {
                Ok(modified) => Some(modified),
                Err(_) => None,
            },
            Err(_) => None,
        } {
            Some(t) => DateTime::from(t),
            None => Utc::now(),
        };

        let new_save = MudrunnerSave {
            original_name: String::from(original_name),
            timestamp: modified,
            user_name: String::from(user_name),
            internal_filename: Some(to.clone()),
        };
        existing_saves.push(new_save);

        // Save the new vecotr to json
        let target_json = match File::create(path) {
            Ok(f) => f,
            Err(e) => {
                dbg!(&e);
                return Err(AppError::FileWriteError(e.to_string()));
            }
        };
        match serde_json::to_writer_pretty(target_json, &existing_saves) {
            Ok(_) => (),
            Err(e) => {
                dbg!(&e);
                return Err(AppError::FileWriteError(e.to_string()));
            }
        }

        Ok(())
    }

    // function to install a specific savegame (overwriting the existing one)
    pub fn restore_savegame(internal_filename: &str, original_name: &str) -> Result<(), AppError> {
        let internal = PathBuf::from(internal_filename);
        let original = PathBuf::from(original_name);

        if let Err(_) = copy(&internal, &original) {
            return Err(AppError::FileWriteError(String::from("Couldn't restore backup.")));
        }
        
        Ok(())
    }
}

impl Clone for MudrunnerSave {
    fn clone(&self) -> MudrunnerSave {
        let new_save = MudrunnerSave {
            user_name: self.user_name.clone(),
            timestamp: self.timestamp,
            original_name: self.original_name.clone(),
            internal_filename: self.internal_filename.clone(),
        };

        return new_save;
    }
}

fn get_mudrunner_data_dir() -> Result<PathBuf, AppError> {
    let mut path = match dirs::config_dir() {
        Some(d) => d,
        None => return Err(AppError::HomeDirNotFound(String::from(""))),
    };
    path.push(DATA_PATH);
    Ok(path)
}

fn get_mudrunner_profile_dir() -> Result<PathBuf, AppError> {
    let mut path = crate::get_app_data_dir();
    path.push(PROFILE_PATH);
    Ok(path)
}
