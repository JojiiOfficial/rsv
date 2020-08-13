use std::error;
use std::process;
use std::time::Duration;

use crate::args::{AppArgs, ListAction, Subcommands};

use config::Config;
use sv::cmdtype::SvCommandType;
use sv::service::Service;

// Run the app
pub fn run(opts: AppArgs) -> Result<String, Box<dyn error::Error>> {
    let config = Config::new()?;

    if let Subcommands::List(list_options) = opts.cmd {
        return run_list_command(list_options, config);
    }

    let (service, cmd_type) = parse_subcommands(opts.cmd, config);

    // Run the actual command
    service.run(cmd_type, Duration::from_secs(opts.timeout.unwrap_or(7)))
}

// parse the subcommands
fn parse_subcommands(cmds: Subcommands, config: Config) -> (Service, SvCommandType) {
    let (action, sv_type) = match cmds {
        Subcommands::Enable(action) => (action, SvCommandType::Enable),
        Subcommands::Disable(action) => (action, SvCommandType::Disable),
        Subcommands::Start(action) => (action, SvCommandType::Up),
        Subcommands::Stop(action) => (action, SvCommandType::Down),
        Subcommands::Status(action) => (action, SvCommandType::Status),
        Subcommands::Restart(action) => (action, SvCommandType::Restart),
        Subcommands::Once(action) => (action, SvCommandType::Once),
        Subcommands::Pause(action) => (action, SvCommandType::Pause),
        Subcommands::Continue(action) => (action, SvCommandType::Continue),
        Subcommands::Term(action) => (action, SvCommandType::Terminate),
        Subcommands::Hup(action) => (action, SvCommandType::Hangup),
        Subcommands::Alarm(action) => (action, SvCommandType::Alarm),
        Subcommands::Interrupt(action) => (action, SvCommandType::Interrupt),
        Subcommands::Kill(action) => (action, SvCommandType::Kill),
        _ => process::exit(1),
    };

    (Service::new(action.service, config), sv_type)
}

pub fn run_list_command(
    list_options: ListAction,
    config: Config,
) -> Result<String, Box<dyn error::Error>> {
    let dir = config.runsv_dir.clone();
    let services = Service::get_all_services(config, &dir)?.into_iter();

    for item in services {
        print!("{}", item.status().unwrap());
    }

    Ok("".to_string())
}

fn format_services(services: Vec<Service>) -> String {
    "".to_string()
}
