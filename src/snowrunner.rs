use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::fs::{read_dir, DirEntry, File};
use std::io::BufReader;
use std::path::PathBuf;
use zip::ZipWriter;

use crate::error::AppError;

const SNOWRUNNER_DATA_DIR: &'static str = "My Games\\SnowRunner\\";
const PROFILE_PATH: &'static str = "base\\storage";
const SAVEGAME_FILE_EXT: &'static str = "dat";
const DATA_FOLDER: &'static str = "sr-data";

#[derive(Debug, Deserialize, Serialize)]
pub struct SnowRunnerSave {
    uuid: String,
    modified: DateTime<Utc>,
    meta_data: SnowRunnerMetaData,
}

#[derive(Debug, Deserialize, Serialize)]
struct SnowRunnerMetaData {
    stored_profiles: Vec<SavedProfile>,
    alias: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SavedProfile {
    name: String,
    saved_on: DateTime<Utc>,
    file_path: String,
}

impl SnowRunnerSave {
    pub fn get_available_snowrunner_saves() -> Result<Vec<SnowRunnerSave>, AppError> {
        let path = get_snowrunner_data_dir()?;
        let dir_list = match read_dir(&path) {
            Ok(d) => d,
            Err(e) => {
                dbg!(&path);
                return Err(AppError::SnowRunnerProfileDirMissing(e.to_string()));
            }
        };

        let mut profile_dirs: Vec<DirEntry> = Vec::new();
        for dir in dir_list {
            if let Ok(d) = dir {
                if is_profile_dir(&d) {
                    profile_dirs.push(d)
                }
            }
        }
        if profile_dirs.is_empty() {
            return Err(AppError::SnowRunnerNoProfile(String::from(
                "No SnowRunner profiles found",
            )));
        }

        let mut profiles = Vec::<SnowRunnerSave>::new();
        for profile in profile_dirs {
            let modified = match match profile.metadata() {
                Ok(meta) => match meta.modified() {
                    Ok(modified) => Some(modified),
                    Err(_) => None,
                },
                Err(_) => None,
            } {
                Some(t) => DateTime::from(t),
                None => Utc::now(),
            };
            if let Some(uuid_osstr) = profile.path().file_name() {
                if let Some(uuid) = uuid_osstr.to_str() {
                    let meta_data = try_load_metadata(uuid);
                    profiles.push(SnowRunnerSave {
                        uuid: String::from(uuid),
                        modified: modified,
                        meta_data: meta_data,
                    })
                }
            }
        }

        Ok(profiles)
    }

    pub fn get_archived_snowrunner_saves() -> Result<Vec<SnowRunnerSave>, AppError> {
        return Err(AppError::Unimplemented);
    }

    fn archive_savegame(&mut self, name: &str) -> Result<(), AppError> {
        let mut path = get_snowrunner_data_dir()?;
        path.push(&self.uuid);
        let files_to_store = match read_dir(path) {
            Ok(files) => files,
            Err(e) => {
                dbg!(e);
                return Err(AppError::FileReadError);
            }
        };

        let archive_name = format!("{}_{}.zip", self.uuid, name);
        // todo: create archive of current profile folder in app-data;
        // add saved profile to metadata
        // store metadata
        Ok(())
    }
}

// TODO: custom data dir via settings if it can not be found with default approach
fn get_snowrunner_data_dir() -> Result<PathBuf, AppError> {
    let mut path = match dirs::document_dir() {
        Some(d) => d,
        None => return Err(AppError::HomeDirNotFound),
    };
    path.push(PROFILE_PATH);
    Ok(path)
}

fn try_load_metadata(profile: &str) -> SnowRunnerMetaData {
    let mut path = match crate::get_app_data_dir() {
        Ok(p) => p,
        Err(e) => {
            dbg!(e);
            panic!("Unable to get app data directory!");
        }
    };
    path.push(DATA_FOLDER);
    path.push(format!("{}.meta", profile));
    dbg!("Loading metadata", &path);
    if let Ok(f) = File::open(path) {
        let reader = BufReader::new(f);

        match serde_json::from_reader::<BufReader<File>, SnowRunnerMetaData>(reader) {
            Ok(meta) => return meta,
            Err(e) => {
                dbg!(e);
            }
        }
    }

    SnowRunnerMetaData {
        stored_profiles: Vec::new(),
        alias: None,
    }
}

fn is_profile_dir(dir: &DirEntry) -> bool {
    if dir.path().is_dir() && dir.file_name().len() == 32 {
        let files = match read_dir(dir.path()) {
            Ok(f) => f,
            Err(e) => {
                dbg!(e);
                return false;
            }
        };
        for file in files {
            if let Ok(file) = file {
                if file.path().is_file() {
                    if let Some(ext) = file.path().extension() {
                        return ext.eq(SAVEGAME_FILE_EXT);
                    }
                }
            }
        }
    }
    return false;
}
