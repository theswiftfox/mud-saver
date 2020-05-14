extern crate config;

use config::ConfigError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    resolution: Resolution,
    color: String,
}

#[derive(Serialize, Deserialize)]
struct Resolution {
    width: u32,
    height: u32,
}

impl Settings {
    pub fn load() -> Result<Settings, ConfigError> {
        let mut conf = config::Config::default();
        conf.merge(config::File::with_name("settings.json"))
            .unwrap();
        conf.try_into::<Settings>()
    }

    pub fn try_load() -> Settings {
        match Settings::load() {
            Ok(s) => s,
            Err(e) => panic!(e),
        }
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        (self.resolution.width, self.resolution.height)
    }

    pub fn get_color(&self) -> String {
        self.color.clone()
    }
}
