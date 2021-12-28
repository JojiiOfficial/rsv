use serde_derive::{Deserialize, Serialize};

use std::env;
use std::error;
use std::fs::{self, create_dir_all, File};
use std::io::{self, stdin, Error, Write};
use std::path::{self, Path};
use std::process;
use sysinfo::{ProcessExt, SystemExt};

pub const DEFAULT_CONF_PATH: &str = "/etc/runitsv/";
pub const DEFAULT_CONF_FILE: &str = "default.conf";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub runsv_dir: String,
    pub service_path: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut service_path = String::new();

        if sys_info::linux_os_release()
            .and_then(|f| {
                let release;

                if f.id.is_some() {
                    release = f.id.unwrap();
                } else if f.name.is_some() {
                    release = f.name.unwrap()
                } else {
                    println!("Release is empty");
                    return Err(sys_info::Error::IO(Error::new(io::ErrorKind::NotFound, "")));
                }

                match release.to_lowercase().as_str() {
                    "artix" => service_path = String::from("/etc/runit/sv/"),
                    "void" => service_path = String::from("/etc/sv/"),
                    // TODO add other runit based distros
                    _ => {
                        return Err(sys_info::Error::IO(Error::new(io::ErrorKind::NotFound, "")));
                    }
                };

                Ok(())
            })
            .is_err()
            || service_path.is_empty()
        {
            println!("Coludn't find your service source path!");
            println!("Enter your service source dir manually (keep empty to skip):");

            // Check if path exists
            loop {
                stdin()
                    .read_line(&mut service_path)
                    .expect("Error reading from stdin");

                if service_path.is_empty() {
                    break;
                }

                service_path = service_path.trim().to_owned();
                if !Path::new(&service_path).exists() {
                    println!("Path does not exists!");
                    service_path = String::new();
                    continue;
                }

                break;
            }
        }

        Config {
            runsv_dir: String::new(),
            service_path,
        }
    }
}

impl Config {
    /// Create a new config
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let conf_path = path::Path::new(DEFAULT_CONF_PATH);
        if !conf_path.exists() {
            #[cfg(feature = "auto_sudo")]
            sudo::escalate_if_needed()?;

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
            #[cfg(feature = "auto_sudo")]
            sudo::escalate_if_needed()?;

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
        if var.is_empty() {
            config.runsv_dir = var;
            return false;
        }
    }

    // Only use config if usable
    if config.runsv_dir.len() > 1 && Path::new(&config.runsv_dir.as_str()).exists() {
        return false;
    }

    let mut sys = sysinfo::System::new();
    sys.refresh_processes();

    if let Some(proc) = sys.process_by_name("runsvdir").into_iter().next() {
        let mut cmd = proc.cmd().iter(); // runsvdir -P <dir> [log]
        if cmd.next().is_some() {
            if let Some(opt) = cmd.next() {
                if opt == "-P" {
                    if let Some(dir) = cmd.next() {
                        if Path::new(dir).is_dir() {
                            config.runsv_dir = dir.clone();
                            return true;
                        }
                    }
                }
            }
        }
    }

    println!("Can't find runsvdir! make sure you have a running 'runsvdir' process!");
    process::exit(1);
}
