use serde::{Deserialize, Serialize};

use std::error;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io::Write;
use std::path;

pub const DEFAULT_CONF_PATH: &str = "/etc/runitsv/";
pub const DEFAULT_CONF_FILE: &str = "default.conf";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    runsv_dir: String,
    service_path: String,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let path = path::Path::new(DEFAULT_CONF_PATH);
        if !path.exists() {
            create_dir_all(path)?;
        }

        let settings: Settings;
        let file = path.join(DEFAULT_CONF_FILE);
        if file.exists() {
            let mut f = File::open(file)?;
            let mut buff = String::new();
            f.read_to_string(&mut buff)?;
            settings = serde_yaml::from_str(&buff)?;
        } else {
            let mut f = File::create(file)?;
            settings = Settings::get_default();
            f.write_all(serde_yaml::to_string(&settings)?.as_bytes())?;
        }

        Ok(settings)
    }

    pub fn get_default() -> Settings {
        Settings {
            runsv_dir: "".to_string(),
            service_path: "/etc/runit/sv".to_string(),
        }
    }
}
