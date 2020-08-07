use clap::{App, AppSettings};

// Constants defining the subcommands
const ENABLE_SUBCOMMAND: &str = "enable";
const DISABLE_SUBCOMMAND: &str = "disable";
const STOP_SUBCOMMAND: &str = "stop";
const START_SUBCOMMAND: &str = "start";

fn main() {
    let matches = setup_cli();

    run(matches);
}

// Init the cli parsing
fn setup_cli<'a>() -> App<'a> {
    let enable_sub = App::new(ENABLE_SUBCOMMAND).about("Enable a service");
    let disable_sub = App::new(DISABLE_SUBCOMMAND).about("Disable a service");
    let stop_sub = App::new(STOP_SUBCOMMAND).about("Stop a service");
    let start_sub = App::new(START_SUBCOMMAND).about("Start a service");

    App::new("runit-helper")
        .version("0.1")
        .author("Jojii <jojii@gmx.net>")
        .arg("-v, --verbose 'show verbose output'")
        .subcommand(enable_sub)
        .subcommand(disable_sub)
        .subcommand(stop_sub)
        .subcommand(start_sub)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColorAlways)
}

// Run the app
fn run(app: App) {
    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some(ENABLE_SUBCOMMAND) => enable_subcommand(&matches),
        _ => println!("Unreachable"),
    }
}

fn enable_subcommand(matches: &clap::ArgMatches) {
    println!("Enableee");
}
