use std::error;
use std::process;

use crate::args::{AppArgs, ServiceAction, Subcommands};

use config::conf;
use sv::cmdtype::SvCommandType;
use sv::service::Service;

// Run the app
pub fn run(opts: AppArgs) -> Result<String, Box<dyn error::Error>> {
    // We need to check the error later since the config
    // might not be required by the given command
    let mut setting_error = None;
    let settings = match conf::Settings::new() {
        Ok(s) => Some(s),
        Err(e) => {
            setting_error = Some(e);
            None
        }
    };

    let (service, cmd_type) = parse_subcommands(opts.cmd, &settings);

    // Check for config errors here
    if let Some(mut settings) = settings {
        // On success save different setting if necessary
        if service.sv_dir != settings.runsv_dir && service.sv_dir.len() > 0 {
            settings.runsv_dir = service.sv_dir.clone();
            settings.save()?;
        }
    } else if let Some(err) = setting_error {
        return Err(err);
    }

    // Run the actual command
    service.run(cmd_type)
}

// parse the subcommands
fn parse_subcommands(
    cmds: Subcommands,
    settings: &Option<conf::Settings>,
) -> (Service, SvCommandType) {
    let (action, sv_type) = match cmds {
        Subcommands::Enable(action) => (action, SvCommandType::Enable),
        Subcommands::Disable(action) => (action, SvCommandType::Disable),
        Subcommands::Start(action) => (action, SvCommandType::Up),
        Subcommands::Stop(action) => (action, SvCommandType::Down),
        Subcommands::Status(action) => (action, SvCommandType::Status),

        _ => {
            println!("not yet implemented");
            process::exit(1);
        }
    };

    return (action_to_service(action, settings), sv_type);
}

// Get service by action
fn action_to_service(action: ServiceAction, settings: &Option<conf::Settings>) -> Service {
    match Service::new(action.service, settings) {
        Ok(service) => service,
        Err(err) => {
            eprint!("{}", err.string());
            process::exit(1);
        }
    }
}
