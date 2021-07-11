use std::collections::HashMap;
use std::env;
use std::error;
use std::fmt;
use std::io::{stdin, Write};
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use sysinfo::{SystemExt,ProcessExt};
use xdg::BaseDirectories;

pub const DEFAULT_CONF_PATH: &str = "/etc/runitsv/";
pub const DEFAULT_CONF_FILE: &str = "default.yml";

// Config hold configuration for multiple runsvdir processes
pub type Config = HashMap<String,RunSvDir>;

// Configuration errors
#[derive(Debug)]
pub enum Error {
    NoConfigError,
    NoPathToConf,
    InitConfigError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.string())
    }
}

impl error::Error for Error {}

impl Error {
    pub fn string(&self) -> String {
        match self {
            Error::NoConfigError => "No valid configuration file provided".to_string(),
            Error::InitConfigError => "Can't get information for configuration".to_string(),
            Error::NoPathToConf => "Can't retrieve path for your configuration file".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunSvDir {
    pub runsv_dir: String,
    pub service_path: String,
}

// get configuration from system and user configs
pub fn get() -> Result<Config,Box<dyn error::Error>> {
    // get system config
    let sys_conf_path = Path::new(DEFAULT_CONF_PATH).join(DEFAULT_CONF_FILE);
    let sys_config = parse_config(sys_conf_path.as_path());

    // get user config
    let user_config_path = get_user_config_path()?;
    let mut user_config = parse_config(user_config_path.as_path());
    // merge configs
    user_config.extend(sys_config.into_iter());

    // if config is empty throw error
    if user_config.len() == 0 {
        return Err(Box::new(Error::NoConfigError))
    }
    return Ok(user_config)

}

// get configuration from path to yaml config file
fn parse_config(config_path: &Path) -> Config {
    // read config to  string
    let config_string = match std::fs::read_to_string(&config_path) {
        Ok(conf_string) => conf_string,
        Err(e) => {
            // return default empty config
            eprintln!("Error hapend when read config '{}': {}",config_path.to_str().unwrap(),e);
            return Config::default();
        }
    };
    // parse string
    match serde_yaml::from_str(&config_string) {
        Ok(conf) => return conf,
        Err(e) => {
            eprintln!("Error happened when parse config '{}': {}",config_path.to_str().unwrap(),e);
            // return default empty config
            return Config::default();
        }
    }
}

// generate yaml configuration file and write it to the disk
pub fn init_config(overwrite: bool) -> Result<String,Box<dyn error::Error>> {

    let mut sys = sysinfo::System::new();
    sys.refresh_processes();

    // Get configuration from the running 'runsvdir' processes
    let config: Config = sys.process_by_name("runsvdir").into_iter().enumerate().filter_map(|(num,proc)|{
        let cmd_args: Vec<String> = proc.cmd().into_iter().cloned().collect();
        let runsv_dir: String;
        // if second argument is "-P" third should be path to service directory
        if cmd_args[1] == "-P".to_string() {
            runsv_dir = cmd_args[2].clone();
        } else { // or it is path to service directory
            runsv_dir = cmd_args[1].clone();
        }
        // get service_path. It should be folder where service linked to
        let mut sv_dir_list = Vec::new(); 
        std::fs::read_dir(Path::new(&runsv_dir)).unwrap().for_each(|dir_entry|
            if let Ok(entry) = dir_entry {
                if entry.file_type().unwrap().is_symlink() {
                    let mut sd = std::fs::read_link(entry.path()).unwrap(); // unwrap should be file since entry came from the system
                    sd.pop(); // get to the parent directory of the (sc) specific Service Directory
                    sv_dir_list.push(sd.to_str().unwrap().into());
                }
            }
        );
        Some((
            format!("Service pool {}", num),
            RunSvDir{
                // for simplicity use one of the services from the list 
                service_path: sv_dir_list.pop().unwrap_or(runsv_dir.clone()),
                runsv_dir
            }
        ))
    }).collect();
    // if NO configuration was successfully parsed throw error
    if config.len() == 0 {
        println!("Can't find runsvdir! Make sure you have a running 'runsvdir' process!");
        return Err(Box::new(Error::InitConfigError));
    } 

    let path = get_user_config_path()?;
    if Path::new(&path).exists() && overwrite == false {
        return Ok(format!("Configuration file already exists {}\nuse '-o' flag to overwrite\n", path.to_str().unwrap()));
    } else if Path::new(&path).exists() && overwrite == true {
        std::fs::File::create(&path)?.write_all(serde_yaml::to_string(&config)?.as_bytes())?;
        Ok(format!("Successful overwrite configuration file {}", path.to_str().unwrap()))
    } else  {
        let mut config_dir=path.clone(); // require to create directory for config
        config_dir.pop();
        std::fs::create_dir_all(config_dir)?;
        std::fs::File::create(&path)?.write_all(serde_yaml::to_string(&config)?.as_bytes())?;
        Ok(format!("Successful write configuration to {}", path.to_str().unwrap()))
    }

}

// get path to user configuration path
fn get_user_config_path() -> Result<PathBuf,Box<dyn error::Error>> {
    let mut path: PathBuf;
    // first try get rsv confifugration file path from environment variable RSV_CONF
    if let Some(value) = env::var_os("RSV_CONFIG") {
       path = PathBuf::from(value);
    } // then try to use XDG_CONFIG_HOME/rsv/rsv.yml
    else if let Ok(x) = BaseDirectories::new() {
       path = x.get_config_home();
       path.push("rsv/rsv.yml");
    } // if XDG unavailable try to retreave config path using enviroment variable HOME
    else if let Some(home) = env::var_os("HOME") { 
       path = PathBuf::from(home);
       path.push(".config/rsv/rsv.yml");
    } else { 
        return Err(Box::new(Error::NoPathToConf))
    }
   return Ok(path)
}
