use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    color: String,
}

const SETTINGS_FILE: &'static str = "settings.json";

pub fn load() -> Result<Settings, Box<dyn Error>> {
    let file = File::open(SETTINGS_FILE)?;
    let reader = BufReader::new(file);

    let set = serde_json::from_reader(reader)?;

    Ok(set)
}

pub fn try_load() -> Settings {
    match load() {
        Ok(s) => s,
        Err(e) => panic!("Loading settings failed: {}", e),
    }
}

impl Settings {
    pub fn store(&self) -> Result<(), Box<dyn Error>> {
        serde_json::to_writer_pretty(File::create(SETTINGS_FILE)?, self)?;

        Ok(())
    }

    pub fn reload(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(SETTINGS_FILE)?;
        let reader = BufReader::new(file);

        let set: Settings = serde_json::from_reader(reader)?;
        self.color = set.color;

        Ok(())
    }

    pub fn get_color(&self) -> String {
        self.color.clone()
    }
}
