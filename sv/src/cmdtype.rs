/// All available Commands
/// for runsv
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
}

impl SvCommandType {
    pub fn value(&self) -> Option<String> {
        let res = match *self {
            SvCommandType::Up => "u".to_string(),
            SvCommandType::Down => "d".to_string(),
            SvCommandType::Once => "o".to_string(),
            SvCommandType::Pause => "p".to_string(),
            SvCommandType::Continue => "c".to_string(),
            SvCommandType::Hangup => "h".to_string(),
            SvCommandType::Alarm => "a".to_string(),
            SvCommandType::Interrupt => "i".to_string(),
            SvCommandType::Quit => "q".to_string(),
            SvCommandType::USR1 => "1".to_string(),
            SvCommandType::USR2 => "2".to_string(),
            SvCommandType::Terminate => "t".to_string(),
            SvCommandType::Kill => "k".to_string(),
            SvCommandType::Exit => "e".to_string(),
            _ => return None,
        };

        Some(res)
    }
}
