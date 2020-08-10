use std::fs;
use std::io::ErrorKind;
use std::time::{Duration, SystemTime};

use crate::error::{Error, WARN};
use crate::service::{Service, ServiceFile};

pub const FINISH: &str = "finish";
pub const RUN: &str = "run";
pub const DOWN: &str = "down";

pub const NORMALLY_UP: &str = "normally up";
pub const NORMALLY_DOWN: &str = "normally down";
pub const PAUSED: &str = "paused";
pub const WANT_UP: &str = "want up";
pub const WANT_DOWN: &str = "want down";
pub const GOT_TERM: &str = "got TERM";

#[derive(Debug)]
pub struct ServiceStatus {
    pub pid: i32,
    pub time: Duration,
    pub state: ServiceState,
    pub normallyup: bool,
    pub paused: bool,
    pub want: Wants,
    pub term: bool,
}

#[derive(PartialEq, Debug)]
pub enum Wants {
    NoWant,
    Up,
    Down,
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

        let normallyup = ServiceStatus::normallyup(service);

        let want = {
            if buff[17] == b'u' {
                Wants::Up
            } else if buff[17] == b'd' {
                Wants::Down
            } else {
                Wants::NoWant
            }
        };

        Ok(ServiceStatus {
            pid,
            time,
            state,
            normallyup,
            paused: buff[16] > 0,
            want: want,
            term: buff[18] > 0,
        })
    }

    /// Check http://smarden.org/runit/runsv.8.html
    fn normallyup(service: &Service) -> bool {
        if let Err(err) = fs::metadata(service.get_file_path(ServiceFile::Down)) {
            if err.kind() == ErrorKind::NotFound {
                return true;
            }

            // On any other error print warning
            // See sv.c:120 from the runit source
            println!(
                "{}: unable to stat {}/down: {}",
                WARN,
                service.uri.clone(),
                err.raw_os_error().unwrap()
            );
        }

        false
    }

    pub fn get_desired_state(&self) -> &str {
        if self.pid > 0 {
            if !self.normallyup {
                return NORMALLY_DOWN;
            }

            if self.paused {
                return PAUSED;
            }

            if self.want == Wants::Down {
                return WANT_DOWN;
            }

            if self.term {
                return GOT_TERM;
            }
        } else {
            if self.normallyup {
                return NORMALLY_UP;
            }

            if self.want == Wants::Up {
                return WANT_UP;
            }
        }

        ""
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

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let sub_sec = Duration::from_secs((time) as u64);

    if sub_sec > now {
        return Duration::from_secs(0);
    }
    return now - sub_sec;
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
