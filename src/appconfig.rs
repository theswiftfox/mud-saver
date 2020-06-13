use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    #[serde(default = "default_color")]
    color: String,
    #[serde(default = "default_enabled")]
    enable_mudrunner: bool,
    #[serde(default = "default_enabled")]
    enable_snowrunner: bool,
}

const SETTINGS_FILE: &'static str = "settings.json";

pub fn load() -> Result<Settings, Box<dyn Error>> {
    let mut settings_path = crate::get_app_data_dir();
    settings_path.push(SETTINGS_FILE);
    let file = File::open(settings_path)?;
    let reader = BufReader::new(file);

    let set = serde_json::from_reader(reader)?;

    Ok(set)
}

pub fn try_load() -> Settings {
    match load() {
        Ok(s) => s,
        Err(e) => {
            dbg!("Loading settings failed: {}", e);
            Settings::default()
        }
    }
}

impl Settings {
    fn default() -> Settings {
        Settings {
            color: default_color(),
            enable_mudrunner: default_enabled(),
            enable_snowrunner: default_enabled(),
        }
    }

    pub fn store(&self) -> Result<(), Box<dyn Error>> {
        dbg!("Stored:", &self);
        let mut settings_path = crate::get_app_data_dir();
        settings_path.push(SETTINGS_FILE);
        serde_json::to_writer_pretty(File::create(settings_path)?, self)?;

        Ok(())
    }

    pub fn reload(&mut self) -> Result<(), Box<dyn Error>> {
        let mut settings_path = crate::get_app_data_dir();
        settings_path.push(SETTINGS_FILE);
        let file = File::open(settings_path)?;
        let reader = BufReader::new(file);

        let set: Settings = serde_json::from_reader(reader)?;
        *self = set;
        Ok(())
    }

    pub fn get_color(&self) -> String {
        self.color.clone()
    }
}

fn default_enabled() -> bool {
    true
}

fn default_color() -> String {
    "teal".to_string()
}
