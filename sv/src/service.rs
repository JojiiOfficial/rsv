use crate::cmdtype;
use crate::error::Error;
use std::path::Path;
use sysinfo::SystemExt;

// A sv command
#[derive(Debug)]
pub struct Service {
    uri: String,
    sv_dir: String,
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

impl Service {
    /// Create a new SvCommand object
    pub fn new(uri: String) -> Result<Service, Error> {
        // Get service directory
        let sv_dir = match get_svdir() {
            Some(v) => v,
            None => return Err(Error::ServiceNotAccessable),
        };

        let service = Service { uri, sv_dir };
        service.check()?;

        Ok(service)
    }

    // Check given service
    fn check(&self) -> Result<(), Error> {
        if !is_path(&self.sv_dir) {
            return Err(Error::ServiceDirNotFound);
        }

        Ok(())
    }

    fn get_file_path(&self, kfile: ServiceFile) -> String {
        String::from("")
    }

    /// Run a sv command
    pub fn run(&self, _cmd: cmdtype::SvCommandType) -> String {
        "".to_string()
    }
}

// Try to get service dir
fn get_svdir() -> Option<String> {
    let sys = sysinfo::System::new();
    let mut was_p = false;

    for (_, v) in sys.get_process_list().iter() {
        if !v.name.contains("runsvdir") {
            continue;
        }

        for arg in &v.cmd {
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
