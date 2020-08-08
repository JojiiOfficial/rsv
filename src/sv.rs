use crate::args::ServiceAction;
use std::process::Command;

struct SvCommand<'a> {
    cmd: &'a str,
    verbose: bool,
}

impl<'a> SvCommand<'a> {
    fn new(saction: &'a ServiceAction) -> SvCommand<'a> {
        SvCommand {
            cmd: saction.service.as_str(),
            verbose: saction.verbose,
        }
    }

    fn start(&self) -> (String, String) {
        self.run_sv_command("start")
    }

    fn stop_sv(&self) -> (String, String) {
        self.run_sv_command("stop")
    }

    fn status_sv(&self) -> (String, String) {
        self.run_sv_command("status")
    }

    fn run_sv_command(&self, arg: &str) -> (String, String) {
        let mut args = vec![arg];
        if self.verbose {
            args.push("-v");
        }
        args.push(self.cmd);

        let v = Command::new("sv").args(args).output().unwrap();

        // Return stdout and stderr inside a tulp
        (
            String::from_utf8(v.stdout).unwrap(),
            String::from_utf8(v.stderr).unwrap(),
        )
    }
}

pub fn print_output(res: (String, String)) {
    let stdout = res.0;
    let stderr = res.1;

    if stdout.len() > 0 {
        print!("{}", stdout);
    }

    if stderr.len() > 0 {
        eprint!("{}", stderr);
    }
}

// Start a service
pub fn start(opts: ServiceAction) {
    let serv = SvCommand::new(&opts);
    print_output(serv.start());
}

// Get status of a service
pub fn status(opts: ServiceAction) {
    let serv = SvCommand::new(&opts);
    print_output(serv.status_sv());
}

// Stop a service
pub fn stop(opts: ServiceAction) {
    let serv = SvCommand::new(&opts);
    print_output(serv.stop_sv());
}
