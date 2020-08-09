use crate::cmdtype::SvCommandType;
use crate::error::{Error, OK};
use crate::status::ServiceStatus;

use std::env;
use std::error;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::{Read, Write};
use std::path::Path;

use config::conf;
use sysinfo::SystemExt;

pub const SRC_DIR: &str = "/etc/runit/sv/";

// A sv command
#[derive(Debug)]
pub struct Service {
    pub uri: String,
    pub sv_dir: String,
}

pub enum ServiceFile {
    // servicedir
    // files
    Run,
    Finish,

    // supervise
    // files
    PID,
    Control,
    Lock,
    OK,
    Stat,
    Status,
}

impl ServiceFile {
    pub fn to_string(&self) -> &str {
        match self {
            ServiceFile::Run => "run",
            ServiceFile::Finish => "finish",
            ServiceFile::PID => "supervise/pid",
            ServiceFile::Control => "supervise/control",
            ServiceFile::Lock => "supervise/lock",
            ServiceFile::OK => "supervise/ok",
            ServiceFile::Stat => "supervise/stat",
            ServiceFile::Status => "supervise/status",
        }
    }
}

impl Service {
    /// Create a new SvCommand object
    pub fn new(uri: String, settings: &conf::Settings) -> Result<Service, Error> {
        // Get service directory
        let sv_dir = match get_svdir(&settings) {
            Some(v) => v,
            None => return Err(Error::DirNotFound(uri.clone())),
        };

        let service = Service { uri, sv_dir };
        service.check()?;

        Ok(service)
    }

    // Check given service
    fn check(&self) -> Result<(), Error> {
        if !is_path(&self.sv_dir) {
            return Err(Error::DirNotFound(self.uri.clone()));
        }

        Ok(())
    }

    pub fn get_file_path(&self, kfile: ServiceFile) -> OsString {
        let a = Path::new(&self.sv_dir)
            .join(&self.uri)
            .join(kfile.to_string());

        return OsString::from(&a.as_os_str());
    }

    /// Run a sv command
    pub fn run(&self, cmd: SvCommandType) -> Result<String, Box<dyn error::Error>> {
        Ok(match cmd {
            SvCommandType::Status => self.status()?,
            SvCommandType::Enable => self.enable(),
            SvCommandType::Disable => self.disable(),

            _ => self.run_control_cmd(cmd),
        })
    }

    pub fn run_control_cmd(&self, cmd: SvCommandType) -> String {
        let control_path = self.get_file_path(ServiceFile::Control);

        let mut f = match OpenOptions::new().write(true).open(&control_path) {
            Ok(file) => file,
            Err(_) => return Error::DirNotFound(self.uri.clone()).string(),
        };

        if let Err(_) = f.write_all(cmd.value().unwrap().as_bytes()) {
            return Error::DirNotFound(self.uri.clone()).string();
        };

        format!("{}: {}:", OK, self.uri)
    }

    pub fn status(&self) -> Result<String, Box<dyn error::Error>> {
        let status = self.read_status()?;
        println!("{:#?}", status);

        Ok("".to_string())
    }

    pub fn enable(&self) -> String {
        "".to_string()
    }

    pub fn disable(&self) -> String {
        "".to_string()
    }

    fn read_status(&self) -> Result<ServiceStatus, Box<dyn error::Error>> {
        let status_path = self.get_file_path(ServiceFile::Status);
        let f = OpenOptions::new().read(true).open(&status_path)?;

        let mut f = BufReader::new(f);
        let mut buff = [0; 20];
        f.read_exact(&mut buff).expect("read error");

        let service = ServiceStatus::new_by_buff(self, buff)?;
        return Ok(service);
    }
}

// Try to get service dir
fn get_svdir(settings: &conf::Settings) -> Option<String> {
    // Check environment variable first
    if let Ok(var) = env::var("SVDIR") {
        return Some(var);
    }

    if settings.runsv_dir.len() > 0 && is_path(&settings.runsv_dir.as_str()) {
        return Some(settings.runsv_dir.clone());
    }

    let sys = sysinfo::System::new();
    let mut was_p = false;

    for (_, v) in sys.get_process_list().iter() {
        if !v.name.contains("runsvdir") {
            continue;
        }

        for arg in v.cmd.iter() {
            if arg == "-P" {
                was_p = true;
                continue;
            }

            if was_p && arg.len() > 0 && arg.starts_with("/") {
                return Some(arg.clone());
            }
        }
    }

    None
}

// return true if given path exists
fn is_path(s: &str) -> bool {
    Path::new(s).exists()
}
