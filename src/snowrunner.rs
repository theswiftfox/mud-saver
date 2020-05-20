use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::fs::{read_dir, DirEntry, File};
use std::io::BufReader;

use crate::error::AppError;

const PROFILE_PATH: &'static str = "Documents/My Games/SnowRunner/base/storage";
const SAVEGAME_FILE_EXT: &'static str = "dat";
const DATA_FOLDER: &'static str = "sr-data";

#[derive(Debug, Deserialize, Serialize)]
pub struct SnowRunnerProfile {
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

impl SnowRunnerProfile {
    pub fn load_profiles_from_disk() -> Result<Vec<SnowRunnerProfile>, AppError> {
        let mut path = match dirs::home_dir() {
            Some(d) => d,
            None => return Err(AppError::HomeDirNotFound(String::from(""))),
        };
        path.push(PROFILE_PATH);
        let dir_list = match read_dir(path) {
            Ok(d) => d,
            Err(e) => return Err(AppError::SRnoProfileFound(e.to_string())),
        };

        let mut profile_dirs: Vec<DirEntry> = Vec::new();
        for dir in dir_list {
            if let Ok(d) = dir {
                if SnowRunnerProfile::is_profile_dir(&d) {
                    profile_dirs.push(d)
                }
            }
        }
        if profile_dirs.is_empty() {
            return Err(AppError::SRnoProfileFound(String::from(
                "No SnowRunner profiles found",
            )));
        }

        let mut profiles = Vec::<SnowRunnerProfile>::new();
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
                    let meta_data = SnowRunnerProfile::try_load_metadata(uuid);
                    profiles.push(SnowRunnerProfile {
                        uuid: String::from(uuid),
                        modified: modified,
                        meta_data: meta_data,
                    })
                }
            }
        }

        Ok(profiles)
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

    fn do_backup(&mut self, name: &str) -> Result<(), AppError> {
        // todo: create archive of current profile folder in app-data;
        // add saved profile to metadata
        // store metadata
        Ok(())
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
}
