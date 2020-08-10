use serde::{Deserialize, Serialize};

use std::env;
use std::error;
use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::{self, Path};
use std::process;

use sysinfo::SystemExt;

pub const DEFAULT_CONF_PATH: &str = "/etc/runitsv/";
pub const DEFAULT_CONF_FILE: &str = "default.conf";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub runsv_dir: String,
    pub service_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            runsv_dir: "".into(),
            service_path: "/etc/runit/sv".into(),
        }
    }
}

impl Config {
    /// Create a new config
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let conf_path = path::Path::new(DEFAULT_CONF_PATH);
        if !conf_path.exists() {
            create_dir_all(conf_path)?;
        }

        let mut settings: Config;
        let mut need_save = false;
        let file = conf_path.join(DEFAULT_CONF_FILE);
        if file.exists() {
            // Read existing config
            settings = serde_yaml::from_str(fs::read_to_string(&file)?.as_str())?;
        } else {
            // Create new config from default value
            settings = Config::default();
            need_save = true;
        }

        if init_svdir(&mut settings) {
            need_save = true
        }

        if need_save {
            settings.save()?;
        }

        Ok(settings)
    }

    /// Save the config
    pub fn save(&self) -> Result<(), Box<dyn error::Error>> {
        File::create(path::Path::new(DEFAULT_CONF_PATH).join(DEFAULT_CONF_FILE))?
            .write_all(serde_yaml::to_string(self)?.as_bytes())?;

        Ok(())
    }
}

fn init_svdir(config: &mut Config) -> bool {
    // Check environment variable first
    if let Ok(var) = env::var("SVDIR") {
        if var.len() > 0 {
            config.runsv_dir = var;
            return false;
        }
    }

    // Only use config if usable
    if config.runsv_dir.len() > 1 && Path::new(&config.runsv_dir.as_str()).exists() {
        return false;
    }

    let sys = sysinfo::System::new();
    let a = sys
        .get_process_list()
        .iter()
        .filter(|(_, v)| v.name.contains("runsvdir"))
        .nth(0);

    let not_found_err = || {
        println!("Can't find runsvdir! make sure you have a running 'runsvdir' process!");
        process::exit(1);
    };

    if let None = a {
        not_found_err();
    }

    let mut was_p = false;
    for arg in a.unwrap().1.cmd.iter() {
        if arg == "-P" {
            was_p = true;
            continue;
        }

        if was_p && arg.len() > 0 && arg.starts_with("/") {
            config.runsv_dir = arg.clone();
            return true;
        }
    }

    not_found_err();
    false
}
