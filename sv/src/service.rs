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

use crate::error::Error as err;
use config::Config;

// A sv command
#[derive(Debug)]
pub struct Service {
    pub uri: String,
    config: Config,
    pub src: ServiceSrc,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ServiceSrc {
    RunSvDir,
    ServiceDir,
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
    pub fn new(uri: String, settings: Config, src: ServiceSrc) -> Service {
        Service {
            uri,
            config: settings,
            src,
        }
    }

    pub fn get_file_path(&self, kfile: ServiceFile) -> OsString {
        let a = Path::new(&self.config.runsv_dir)
            .join(&self.uri)
            .join(kfile.to_string());

        OsString::from(&a.as_os_str())
    }

    pub fn get_all_services(config: Config) -> Result<Vec<Self>, Box<dyn error::Error>> {
        let mut services: Vec<Self> = Vec::new();

        for (dir, src) in [
            (&config.runsv_dir, ServiceSrc::RunSvDir),
            (&config.service_path, ServiceSrc::ServiceDir),
        ]
        .iter()
        {
            let dir_entries = match fs::read_dir(&dir) {
                Ok(de) => de,
                Err(_) => continue,
            };

            for item in dir_entries {
                if item.is_err() {
                    return Err(Box::new(item.err().unwrap()));
                }

                let service_uri = item.unwrap().file_name().into_string().unwrap();

                if services.iter().find(|s| s.uri == service_uri).is_some() {
                    continue;
                }

                services.push(Service::new(service_uri, config.clone(), src.clone()));
            }
        }

        Ok(services)
    }

    /// Run a sv command
    pub fn run(
        &self,
        cmd: SvCommandType,
        timeout: Duration,
    ) -> Result<String, Box<dyn error::Error>> {
        self.check_exists()?;

        Ok(match cmd {
            SvCommandType::Status => self.status(),
            SvCommandType::Enable => self.enable(),
            SvCommandType::Disable => self.disable(),
            SvCommandType::Restart => self.restart(timeout),
            SvCommandType::Up => self.start(timeout, true),

            _ => self.control(cmd, timeout, true),
        }?)
    }

    pub fn start(
        &self,
        timeout: Duration,
        kill_on_timeout: bool,
    ) -> Result<String, Box<dyn error::Error>> {
        self.check_exists()?;

        if !self.is_enabled() {
            self.enable()?;
            sleep(Duration::from_millis(200));
        }

        self.control(SvCommandType::Up, timeout, kill_on_timeout)
    }

    pub fn control(
        &self,
        cmd: SvCommandType,
        timeout: Duration,
        kill_on_timeout: bool,
    ) -> Result<String, Box<dyn error::Error>> {
        let pre = self.run_control_cmd(cmd, timeout, kill_on_timeout)?;
        Ok(format!("{}: {}", pre, self.status()?))
    }

    pub fn run_control_cmd(
        &self,
        cmd: SvCommandType,
        timeout: Duration,
        kill_on_timeout: bool,
    ) -> Result<String, Box<dyn error::Error>> {
        // Write control char into the
        // control file of the service
        fs::OpenOptions::new()
            .write(true)
            .open(self.get_file_path(ServiceFile::Control))?
            .write_all(cmd.value().unwrap().as_bytes())?;

        // Wait for the command to take effect
        // print the result
        Ok(self.await_command(cmd, timeout, kill_on_timeout)?)
    }

    fn await_command(
        &self,
        cmd: SvCommandType,
        timeout: Duration,
        kill_on_timeout: bool,
    ) -> Result<String, Box<dyn error::Error>> {
        let end = SystemTime::now().add(timeout);
        loop {
            sleep(Duration::from_millis(40));

            if end < SystemTime::now() {
                if kill_on_timeout {
                    return self.control(SvCommandType::Kill, timeout, false);
                }
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

    pub fn restart(&self, timeout: Duration) -> Result<String, Box<dyn error::Error>> {
        let status = self.read_status()?;
        if status.state != ServiceState::Down {
            self.run_control_cmd(SvCommandType::Down, timeout, true)?;
        }

        self.run_control_cmd(SvCommandType::Up, timeout, true)?;
        sleep(Duration::from_millis(500));

        Ok(format!("ok: {}", self.status()?))
    }

    pub fn status(&self) -> Result<String, Box<dyn error::Error>> {
        self.check_enabled()?;
        Ok(self.format_status(self.read_status()?))
    }

    pub fn format_status(&self, status: ServiceStatus) -> String {
        let mut fmt: String = format!(
            "{}: {}: (pid {}) {}s",
            status.state.value(),
            self.uri,
            status.pid,
            status.time.as_secs()
        );

        let desired_state = status.get_desired_state();
        if desired_state.is_empty() {
            fmt.push_str(&desired_state);
        }

        fmt.push('\n');
        fmt
    }

    pub fn enable(&self) -> Result<String, Box<dyn error::Error>> {
        self.check_exists()?;
        self.check_already_enabled()?;

        ufs::symlink(
            Path::new(&self.config.service_path).join(&self.uri),
            Path::new(&self.config.runsv_dir).join(&self.uri),
        )?;

        Ok(format!("Service '{}' enabled successfully\n", self.uri))
    }

    pub fn disable(&self) -> Result<String, Box<dyn error::Error>> {
        self.check_exists()?;

        if !self.is_enabled() {
            return Err(Box::new(err::ServiceAlreadyDisabled(self.uri.clone())));
        }

        fs::remove_file(Path::new(&self.config.runsv_dir).join(&self.uri))?;
        Ok(format!("Service '{}' disabled successfully\n", self.uri))
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.config.service_path)
            .join(&self.uri)
            .exists()
    }

    pub fn is_enabled(&self) -> bool {
        Path::new(&self.config.runsv_dir).join(&self.uri).exists()
    }

    pub fn read_status(&self) -> Result<ServiceStatus, err> {
        if self.src == ServiceSrc::ServiceDir {
            return Ok(ServiceStatus::no_state_available());
        }

        let status_path = self.get_file_path(ServiceFile::Status);
        let f = match fs::OpenOptions::new().read(true).open(&status_path) {
            Ok(file) => file,
            Err(error) => return Err(err::IoError(error)),
        };

        let mut f = BufReader::new(f);
        let mut buff = [0; 20];
        f.read_exact(&mut buff).expect("read error");

        let service = ServiceStatus::new(self, buff)?;
        Ok(service)
    }

    fn check_already_enabled(&self) -> Result<(), Box<dyn error::Error>> {
        if self.is_enabled() {
            return Err(Box::new(err::ServiceAlreadyEnabled(self.uri.clone())));
        }
        Ok(())
    }

    fn check_exists(&self) -> Result<(), Box<dyn error::Error>> {
        if !self.exists() {
            return Err(Box::new(err::ServiceNotFound(self.uri.clone())));
        }
        Ok(())
    }

    fn check_enabled(&self) -> Result<(), err> {
        if !self.is_enabled() {
            return Err(err::ServiceNotEnabled(self.uri.clone()));
        }
        Ok(())
    }
}
