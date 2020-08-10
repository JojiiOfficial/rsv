use std::fs;
use std::io::ErrorKind;
use std::time::{Duration, SystemTime};

use crate::error::{Error, WARN};
use crate::service::{Service, ServiceFile};

pub const FINISH: &str = "finish";
pub const RUN: &str = "run";
pub const DOWN: &str = "down";

#[derive(Debug)]
pub struct ServiceStatus {
    pub pid: i32,
    pub time: Duration,
    pub state: ServiceState,
    pub normallyup: bool,
}

#[derive(PartialEq, Debug)]
pub enum ServiceState {
    Down,
    Run,
    Finish,
}

impl ServiceState {
    pub fn value(&self) -> &str {
        match self {
            ServiceState::Down => DOWN,
            ServiceState::Run => RUN,
            ServiceState::Finish => FINISH,
        }
    }
}

impl ServiceStatus {
    pub fn new_by_buff(service: &Service, buff: [u8; 20]) -> Result<ServiceStatus, Error> {
        let time = parse_time(&buff);
        let pid = parse_pid(&buff);

        // Parse running status
        let state = match buff[19] {
            0 => ServiceState::Down,
            1 => ServiceState::Run,
            2 => ServiceState::Finish,

            _ => return Err(Error::ParsingStatus(service.uri.clone())),
        };

        // Check http://smarden.org/runit/runsv.8.html
        let normallyup = match fs::metadata(service.get_file_path(ServiceFile::Down)) {
            Ok(_) => false,
            Err(err) => match err.kind() {
                // If simply not found, the service is normally up
                ErrorKind::NotFound => true,

                // On any other error print warning
                // See sv.c:120 from the runit source
                _ => {
                    println!(
                        "{}: unable to stat {}/down: {}",
                        WARN,
                        service.uri.clone(),
                        err.raw_os_error().unwrap()
                    );
                    false
                }
            },
        };

        Ok(ServiceStatus {
            pid,
            time,
            state,
            normallyup,
        })
    }

    pub fn is_running(&self) -> bool {
        return self.state == ServiceState::Run;
    }
}

fn parse_time(buff: &[u8; 20]) -> Duration {
    let mut time: u32 = buff[0] as u32;
    time <<= 8;
    time += buff[1] as u32;
    time <<= 8;
    time += buff[2] as u32;
    time <<= 8;
    time += buff[3] as u32;
    time <<= 8;
    time += buff[4] as u32;
    time <<= 8;
    time += buff[5] as u32;
    time <<= 8;
    time += buff[6] as u32;
    time <<= 8;
    time += buff[7] as u32;

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        - Duration::from_secs((time - 10) as u64)
}

fn parse_pid(buff: &[u8; 20]) -> i32 {
    let mut pid: i32;
    pid = buff[15] as i32;
    pid <<= 8;
    pid += buff[14] as i32;
    pid <<= 8;
    pid += buff[13] as i32;
    pid <<= 8;
    pid += buff[12] as i32;
    pid
}
