use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::fs::{read_dir, DirEntry, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use crate::error::AppError;

const PROFILE_PATH: &'static str = "My Games\\SnowRunner\\base\\storage";
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SavedProfile {
    name: String,
    saved_on: DateTime<Utc>,
    file_path: PathBuf,
    uuid: String,
}

impl SnowRunnerMetaData {
    pub fn default() -> SnowRunnerMetaData {
        SnowRunnerMetaData {
            stored_profiles: Vec::new(),
            alias: None,
        }
    }

    fn remove_deleted_backups(&mut self) {
        // clear saves that are not found on disk
        self.stored_profiles
            .retain(|profile| File::open(&profile.file_path).is_ok());
    }

    fn store(&self, uuid: &str) -> Result<(), AppError> {
        let mut path = match get_snowrunner_data_dir() {
            Ok(p) => p,
            Err(e) => {
                return Err(e);
            }
        };
        path.push(format!("{}.meta", uuid));
        let file = match File::create(path) {
            Ok(f) => f,
            Err(e) => {
                dbg!(&e);
                return Err(AppError::FileCreateError(e.to_string()));
            }
        };
        match serde_json::to_writer_pretty(file, &self) {
            Ok(_) => (),
            Err(e) => {
                dbg!(&e);
                return Err(AppError::FileWriteError(e.to_string()));
            }
        }

        Ok(())
    }
}

impl SnowRunnerProfile {
    pub fn get_snowrunner_profile(profile_id: &str) -> Result<SnowRunnerProfile, AppError> {
        let profiles = SnowRunnerProfile::get_available_snowrunner_profiles()?;
        for profile in profiles {
            if profile.uuid.eq(profile_id) {
                return Ok(profile);
            }
        }
        Err(AppError::SnowRunnerNoProfile(String::from(format!(
            "Profile with id '{}' not found",
            profile_id
        ))))
    }

    pub fn get_available_snowrunner_profiles() -> Result<Vec<SnowRunnerProfile>, AppError> {
        let path = get_snowrunner_profile_dir()?;
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
                    let meta_data = try_load_metadata(uuid);
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

    pub fn get_archived_snowrunner_saves(&self) -> Vec<SavedProfile> {
        self.meta_data.stored_profiles.clone()
    }

    pub fn archive_savegame(&mut self, name: &str) -> Result<(), AppError> {
        let mut path = get_snowrunner_profile_dir()?;
        path.push(&self.uuid);
        let files_to_store = match read_dir(path) {
            Ok(files) => files,
            Err(e) => {
                dbg!(e);
                return Err(AppError::FileReadError(String::new()));
            }
        };
        let uuid = uuid::Uuid::new_v4();
        let archive_name = format!("{}.zip", uuid);
        let mut target = get_snowrunner_data_dir()?;
        target.push(&archive_name);

        let file = match File::create(&target) {
            Ok(f) => f,
            Err(e) => {
                dbg!(&e);
                return Err(AppError::FileCreateError(e.to_string()));
            }
        };
        let write_zip = || -> Result<(), Box<dyn std::error::Error>> {
            let writer = BufWriter::new(file);
            let mut zip = zip::ZipWriter::new(writer);

            let options = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);

            for entry in files_to_store {
                if let Ok(f) = entry {
                    let name_osstr = f.file_name();
                    let name = name_osstr.to_str();
                    if name.is_none() {
                        return Err(Box::new(AppError::FileReadError(String::new())));
                    }
                    let mut source = File::open(f.path())?;

                    let mut buf = Vec::new();
                    source.read_to_end(&mut buf)?;
                    zip.start_file(name.unwrap(), options)?;
                    zip.write_all(&buf)?;
                }
            }

            match zip.finish() {
                Ok(_) => Ok(()),
                Err(e) => {
                    dbg!(&e);
                    Err(Box::new(AppError::FileWriteError(String::from(
                        "finalizing zip file failed!",
                    ))))
                }
            }
        };

        if let Err(e) = write_zip() {
            // delete file
            dbg!(e);
            return Err(AppError::FileWriteError(String::from(
                "Unable to archive savegame!",
            )));
        }

        let saved_profile = SavedProfile {
            name: String::from(name),
            saved_on: Utc::now(),
            file_path: target,
            uuid: uuid.to_simple().to_string(),
        };
        self.meta_data.stored_profiles.push(saved_profile);
        self.store_metadata()?;
        // todo: create archive of current profile folder in app-data;
        // add saved profile to metadata
        // store metadata
        Ok(())
    }

    pub fn delete_archived_savegame(&mut self, uuid: &str) -> Result<(), AppError> {
        let delete_save = || -> Result<(), AppError> {
            let save = self
                .meta_data
                .stored_profiles
                .iter()
                .find(|&sp| sp.uuid.eq(uuid));
            if save.is_none() {
                return Err(AppError::SavegameNotFound(String::from(uuid)));
            }
            match std::fs::remove_file(&save.unwrap().file_path) {
                Ok(_) => Ok(()),
                Err(_) => Err(AppError::FileWriteError(String::from("Delete failed"))),
            }
        };
        match delete_save() {
            Ok(_) => {
                self.meta_data.stored_profiles.retain(|sp| sp.uuid.ne(uuid));
                self.meta_data.store(&self.uuid)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn restore_backup(&self, save_uuid: &str) -> Result<(), AppError> {
        Err(AppError::Unimplemented(String::from("restore_backup")))
    }

    fn store_metadata(&self) -> Result<(), AppError> {
        self.meta_data.store(&self.uuid)
    }
}

fn get_snowrunner_profile_dir() -> Result<PathBuf, AppError> {
    let mut path = match dirs::document_dir() {
        Some(d) => d,
        None => return Err(AppError::HomeDirNotFound(String::new())),
    };
    dbg!(&path);
    path.push(PROFILE_PATH);
    Ok(path)
}

fn get_snowrunner_data_dir() -> Result<PathBuf, AppError> {
    let mut path = match crate::get_app_data_dir() {
        Ok(p) => p,
        Err(e) => {
            dbg!(e);
            return Err(AppError::AppDataDirNotFound(String::new()));
        }
    };
    path.push(DATA_FOLDER);
    Ok(path)
}

fn try_load_metadata(profile: &str) -> SnowRunnerMetaData {
    let mut path = match get_snowrunner_data_dir() {
        Ok(p) => p,
        Err(e) => {
            dbg!(e);
            return SnowRunnerMetaData::default();
        }
    };
    path.push(format!("{}.meta", profile));
    dbg!("Loading metadata", &path);
    if let Ok(f) = File::open(path) {
        let reader = BufReader::new(f);

        match serde_json::from_reader::<BufReader<File>, SnowRunnerMetaData>(reader) {
            Ok(mut meta) => {
                meta.remove_deleted_backups();
                match meta.store(profile) {
                    Ok(_) => return meta,
                    Err(e) => {
                        dbg!(e);
                    }
                }
            }
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
