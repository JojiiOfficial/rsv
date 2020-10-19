use clap::{crate_version, App, AppSettings, Arg};

fn get_base_app_struct<S: AsRef<str>>(name: S, about: &'static str) -> App<'static> {
    App::new(name.as_ref().to_string())
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::ColoredHelp)
        .about(about.as_ref())
}

fn get_service_subcommand<S: AsRef<str>>(name: S, about: &'static str) -> App<'static> {
    get_base_app_struct(name, about)
        .about(about.as_ref())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("service")
                .about("Specify the service")
                .required(true)
                .takes_value(true),
        )
}

pub fn get_cli() -> App<'static> {
    get_base_app_struct(
        "rsv",
        "A tool to maintain runit services like systemd services",
    )
    .setting(AppSettings::ArgRequiredElseHelp)
    .version(crate_version!())
    .author("Jojii S")
    .arg(Arg::new("verbose").short('v').long("verbose").global(true))
    .arg(
        Arg::new("timeout")
            .short('t')
            .long("timeout")
            .global(true)
            .takes_value(true),
    )
    .subcommand(get_service_subcommand("enable", "Enable a service"))
    .subcommand(get_service_subcommand("disable", "Disable a service"))
    .subcommand(get_service_subcommand("start", "Start a service"))
    .subcommand(get_service_subcommand("stop", "Stop a service"))
    .subcommand(get_service_subcommand("restart", "Restart a service"))
    .subcommand(get_service_subcommand(
        "status",
        "Get the status of a service",
    ))
    .subcommand(get_service_subcommand(
        "once",
        "Start if service is not running. Do not restart if it stops",
    ))
    .subcommand(get_service_subcommand(
        "pause",
        "Send SIGSTOP if the service is running",
    ))
    .subcommand(get_service_subcommand(
        "continue",
        "Send SIGCONT if the service is running",
    ))
    .subcommand(get_service_subcommand(
        "term",
        "Send SIGTERM if the service is running",
    ))
    .subcommand(get_service_subcommand(
        "hup",
        "Send SIGHUP if the service is running",
    ))
    .subcommand(get_service_subcommand(
        "alarm",
        "Send SIGALARM if the service is running",
    ))
    .subcommand(get_service_subcommand(
        "interrupt",
        "Send SIGINT if the service is running",
    ))
    .subcommand(get_service_subcommand(
        "kill",
        "Send SIGKILL if the service is running",
    ))
    .subcommand(
        get_base_app_struct("list", "List services")
            .arg(Arg::new("all").long("all").short('a'))
            .arg(Arg::new("all").long("all").short('a'))
            .arg(Arg::new("up").long("up").short('u'))
            .arg(Arg::new("down").long("down"))
            .arg(Arg::new("enabled").long("enabled").short('e'))
            .arg(Arg::new("disabled").long("disabled").short('d')),
    )
}
