use std::error;
use std::process;
use std::time::Duration;

use crate::args::{AppArgs, ListAction, Subcommands};

use config::Config;
use sv::cmdtype::SvCommandType;
use sv::service::{Service, ServiceSrc};
use sv::status::ServiceState;

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

    (
        Service::new(action.service, config, ServiceSrc::RunSvDir),
        sv_type,
    )
}

pub fn run_list_command(
    list_options: ListAction,
    config: Config,
) -> Result<String, Box<dyn error::Error>> {
    Ok(format_services(
        Service::get_all_services(config)?
            .into_iter()
            .filter(|f| match f.read_status() {
                Ok(status) => {
                    if list_options.down && status.state != ServiceState::Down {
                        return false;
                    }

                    if list_options.enabled && f.src == ServiceSrc::ServiceDir {
                        return false;
                    }

                    if list_options.disabled && f.src == ServiceSrc::RunSvDir {
                        return false;
                    }

                    if list_options.up && status.state != ServiceState::Run {
                        return false;
                    }

                    true
                }
                Err(err) => {
                    eprintln!("'{}', {}", f.uri, err);
                    false
                }
            })
            .collect(),
    ))
}

fn format_services(services: Vec<Service>) -> String {
    let mut s = String::new();

    for item in services {
        let status = item.read_status();

        if !status.is_ok() {
            return format!("{}", status.err().unwrap());
        }

        s.push_str(format!("{}", item.format_status(status.unwrap())).as_str());
    }

    s
}
