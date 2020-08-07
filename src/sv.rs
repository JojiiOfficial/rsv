use std::process::Command;

pub fn start(cmd: &str) -> (String, String) {
    run_sv_command(&["start", cmd])
}

pub fn stop(cmd: &str) -> (String, String) {
    run_sv_command(&["stop", cmd])
}

pub fn status(cmd: &str) -> (String, String) {
    run_sv_command(&["status", cmd])
}

fn run_sv_command(args: &[&str]) -> (String, String) {
    let v = Command::new("sv").args(args).output().unwrap();

    (
        String::from_utf8(v.stdout).unwrap(),
        String::from_utf8(v.stderr).unwrap(),
    )
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
