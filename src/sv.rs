use crate::args::ServiceAction;

// A sv command
struct SvCommand<'a> {
    service: &'a str,
    cmd: SvCommandType,
}

impl<'a> SvCommand<'a> {
    /// Create a new SvCommand object
    fn new(cmd: SvCommandType, service: &'a str) -> SvCommand<'a> {
        SvCommand { service, cmd }
    }

    /// Run the sv command
    fn run(&self) -> String {
        "".to_string()
    }
}

/// All available Commands
/// for runsv
pub enum SvCommandType {
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
}

impl SvCommandType {
    fn value(&self) -> String {
        match *self {
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
        }
    }
}

// Start a service
pub fn start(opts: ServiceAction) {
    let serv = SvCommand::new(SvCommandType::Up, opts.service.as_str());
}

// Get status of a service
pub fn status(opts: ServiceAction) {
    // TODO
}

// Stop a service
pub fn stop(opts: ServiceAction) {
    let serv = SvCommand::new(SvCommandType::Down, opts.service.as_str());
}
