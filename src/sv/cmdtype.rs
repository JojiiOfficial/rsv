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
