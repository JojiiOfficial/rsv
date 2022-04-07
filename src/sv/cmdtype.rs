use crate::args::Command;

/// All available Commands
/// for runsv
#[derive(Debug)]
pub enum SvCommandType {
    // Runit commands
    Up,
    Down,
    Once,
    Pause,
    Continue,
    Hangup,
    Alarm,
    Interrupt,
    Quit,
    USR1,
    USR2,
    Terminate,
    Kill,
    Exit,

    // Custom commands
    Disable,
    Enable,
    Status,
    Restart,
}

impl SvCommandType {
    pub fn value(&self) -> Option<&str> {
        let res = match *self {
            SvCommandType::Up => "u",
            SvCommandType::Down => "d",
            SvCommandType::Once => "o",
            SvCommandType::Pause => "p",
            SvCommandType::Continue => "c",
            SvCommandType::Hangup => "h",
            SvCommandType::Alarm => "a",
            SvCommandType::Interrupt => "i",
            SvCommandType::Quit => "q",
            SvCommandType::USR1 => "1",
            SvCommandType::USR2 => "2",
            SvCommandType::Terminate => "t",
            SvCommandType::Kill => "k",
            SvCommandType::Exit => "e",
            _ => return None,
        };

        Some(res)
    }
}

impl From<&str> for SvCommandType {
    fn from(s: &str) -> SvCommandType {
        match s {
            "enable" => SvCommandType::Enable,
            "disable" => SvCommandType::Disable,
            "start" => SvCommandType::Up,
            "stop" => SvCommandType::Down,
            "status" => SvCommandType::Status,
            "restart" => SvCommandType::Restart,
            "once" => SvCommandType::Once,
            "pause" => SvCommandType::Pause,
            "continue" => SvCommandType::Continue,
            "term" => SvCommandType::Terminate,
            "hup" => SvCommandType::Hangup,
            "alarm" => SvCommandType::Alarm,
            "interrupt" => SvCommandType::Interrupt,
            "kill" => SvCommandType::Kill,
            _ => unreachable!(),
        }
    }
}

impl From<&Command> for SvCommandType {
    fn from(command: &Command) -> Self {
        match command {
            Command::Enable { .. } => SvCommandType::Enable,
            Command::Disable { .. } => SvCommandType::Disable,
            Command::Start { .. } => SvCommandType::Up,
            Command::Stop { .. } => SvCommandType::Down,
            Command::Restart { .. } => SvCommandType::Restart,
            Command::Status { .. } => SvCommandType::Status,
            Command::Once { .. } => SvCommandType::Once,
            Command::Pause { .. } => SvCommandType::Pause,
            Command::Continue { .. } => SvCommandType::Continue,
            Command::Term { .. } => SvCommandType::Terminate,
            Command::Hup { .. } => SvCommandType::Hangup,
            Command::Alarm { .. } => SvCommandType::Alarm,
            Command::Interrupt { .. } => SvCommandType::Interrupt,
            Command::Kill { .. } => SvCommandType::Kill,
            _ => unreachable!(),
        }
    }
}
