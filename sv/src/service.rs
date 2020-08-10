use crate::cmdtype::SvCommandType;
use crate::status::{ServiceState, ServiceStatus};

use std::error;
use std::ffi::OsString;
use std::fs;
use std::io::BufReader;
use std::io::{Read, Write};
use std::ops::Add;
use std::os::unix::fs as ufs;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use config::Config;

// A sv command
#[derive(Debug)]
pub struct Service {
    pub uri: String,
    config: Config,
}

pub enum ServiceFile {
    // servicedir
    // files
    Run,
    Down,
    Finish,

    // supervise
    // files
    PID,
    Control,
    Lock,
    OK,
    Stat,
    Status,
    Check,
}

impl ServiceFile {
    pub fn to_string(&self) -> &str {
        match self {
            ServiceFile::Run => "run",
            ServiceFile::Down => "down",
            ServiceFile::Finish => "finish",
            ServiceFile::Check => "supervise/check",
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
    pub fn new(uri: String, settings: Config) -> Service {
        Service {
            uri,
            config: settings.clone(),
        }
    }

    pub fn get_file_path(&self, kfile: ServiceFile) -> OsString {
        let a = Path::new(&self.config.runsv_dir)
            .join(&self.uri)
            .join(kfile.to_string());

        return OsString::from(&a.as_os_str());
    }

    /// Run a sv command
    pub fn run(
        &self,
        cmd: SvCommandType,
        timeout: Duration,
    ) -> Result<String, Box<dyn error::Error>> {
        Ok(match cmd {
            SvCommandType::Status => self.status()?,
            SvCommandType::Enable => self.enable(),
            SvCommandType::Disable => self.disable(),

            _ => self.run_control_cmd(cmd, timeout)?,
        })
    }

    pub fn run_control_cmd(
        &self,
        cmd: SvCommandType,
        timeout: Duration,
    ) -> Result<String, Box<dyn error::Error>> {
        // Write control char into the
        // control file of the service
        fs::OpenOptions::new()
            .write(true)
            .open(self.get_file_path(ServiceFile::Control))?
            .write_all(cmd.value().unwrap().as_bytes())?;

        // Wait for the command to take effect
        // print the result
        print!("{}", self.await_command(cmd, timeout)?);
        Ok(self.status()?)
    }

    fn await_command(
        &self,
        cmd: SvCommandType,
        timeout: Duration,
    ) -> Result<String, Box<dyn error::Error>> {
        let end = SystemTime::now().add(timeout);
        loop {
            sleep(Duration::from_millis(40));

            if end < SystemTime::now() {
                return Ok("timeout".to_string());
            }

            let status = self.read_status()?;
            match cmd {
                SvCommandType::Up => {
                    if (status.pid > 0 && status.state == ServiceState::Run)
                        && ServiceStatus::check_script(self)
                    {
                        break;
                    }
                }
                SvCommandType::Down | SvCommandType::Kill | SvCommandType::Exit => {
                    if status.pid == 0 && status.state == ServiceState::Down {
                        break;
                    }
                }

                _ => break,
            }
        }

        Ok("ok".to_string())
    }

    pub fn status(&self) -> Result<String, Box<dyn error::Error>> {
        let status = self.read_status()?;

        let mut fmt: String = format!(
            "{}: {}: (pid {}) {}s",
            status.state.value(),
            self.uri,
            status.pid,
            status.time.as_secs()
        );

        let desired_state = status.get_desired_state();
        if desired_state.len() > 0 {
            fmt.push_str(&desired_state);
        }

        fmt.push('\n');
        Ok(fmt)
    }

    pub fn enable(&self) -> String {
        if !self.exists() {
            return format!("Service '{}' not found", self.uri);
        }

        if self.is_enabled() {
            return "Service is already enabled".to_string();
        }

        // create symlink
        let symlink = ufs::symlink(
            Path::new(&self.config.service_path).join(&self.uri),
            Path::new(&self.config.runsv_dir).join(&self.uri),
        );
        if let Err(e) = symlink {
            format!("Error: {}", e);
        }

        format!("Service '{}' enabled successfully", self.uri)
    }

    pub fn disable(&self) -> String {
        if !self.exists() {
            return format!("Service '{}' not found", self.uri);
        }

        if !self.is_enabled() {
            return "Service is already disabled".to_string();
        }

        let sv_path = Path::new(&self.config.runsv_dir).join(&self.uri);
        if let Err(e) = fs::remove_file(sv_path) {
            return format!("Err: {}", e);
        }

        format!("Service '{}' disabled successfully", self.uri)
    }

    pub fn exists(&self) -> bool {
        return Path::new(&self.config.service_path)
            .join(&self.uri)
            .exists();
    }

    pub fn is_enabled(&self) -> bool {
        Path::new(&self.config.runsv_dir).join(&self.uri).exists()
    }

    fn read_status(&self) -> Result<ServiceStatus, Box<dyn error::Error>> {
        let status_path = self.get_file_path(ServiceFile::Status);
        let f = fs::OpenOptions::new().read(true).open(&status_path)?;

        let mut f = BufReader::new(f);
        let mut buff = [0; 20];
        f.read_exact(&mut buff).expect("read error");

        let service = ServiceStatus::new(self, buff)?;
        return Ok(service);
    }
}
