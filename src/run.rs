use std::error;
use std::process;
use std::time::Duration;

use crate::args::{AppArgs, Subcommands};

use config::Config;
use sv::cmdtype::SvCommandType;
use sv::service::Service;

// Run the app
pub fn run(opts: AppArgs) -> Result<String, Box<dyn error::Error>> {
    let config = Config::new()?;

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

        _ => {
            println!("not yet implemented");
            process::exit(1);
        }
    };

    return (Service::new(action.service, config), sv_type);
}
