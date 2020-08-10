use std::error;
use std::fmt;

pub const FATAL: &str = "fatal";
pub const FAIL: &str = "fail";
pub const WARN: &str = "warning";
pub const OK: &str = "ok";
pub const TIMEOUT: &str = "timeout";
pub const KILL: &str = "kill";

#[derive(Debug)]
pub enum Error {
    DirNotFound(String),
    ParsingStatus(String),
    SuperviseAccessDenied(String),
    Timeout(String),
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
            Error::DirNotFound(s) => format!(
                "{}: {}: unable to change to service directory: file does not exist",
                FAIL, s
            ),
            Error::SuperviseAccessDenied(s) => format!(
                "{}: {}: unable to open supervise/ok: access denied",
                FAIL, s
            ),
            Error::ParsingStatus(s) => format!("{}: {}: unable to parse Status", FAIL, s),
            Error::Timeout(_) => format!("{}:", TIMEOUT),
        }
    }
}
