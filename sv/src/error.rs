pub const FATAL: &str = "fatal";
pub const FAIL: &str = "fail";
pub const WARN: &str = "warning";
pub const OK: &str = "ok";
pub const RUN: &str = "run";
pub const FINISH: &str = "finish";
pub const DOWN: &str = "down";
pub const TIMEOUT: &str = "timeout";
pub const KILL: &str = "kill";

#[derive(Debug)]
pub enum Error {
    DirNotFound(String),
    ParsingStatus(String),
    SuperviseAccessDenied(String),
}

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
        }
    }
}
