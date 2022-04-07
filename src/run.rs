use std::error;
use std::time::Duration;

use crate::args::{Cli, Command, ListArgs};
use crate::config::Config;
use crate::sv::cmdtype::SvCommandType;
use crate::sv::service::{Service, ServiceSrc};
use crate::sv::status::ServiceState;

// Run the app
pub fn run(cli: Cli) -> Result<String, Box<dyn error::Error>> {
    let config = Config::new()?;

    #[cfg(feature = "auto_sudo")]
    sudo::escalate_if_needed()?;

    if let Command::List(args) = cli.command {
        return run_list_command(config, args);
    }

    let sv_command_type = SvCommandType::from(&cli.command);
    let service_name = match cli.command {
        Command::Enable { service }
        | Command::Disable { service }
        | Command::Start { service }
        | Command::Stop { service }
        | Command::Restart { service }
        | Command::Status { service }
        | Command::Once { service }
        | Command::Pause { service }
        | Command::Continue { service }
        | Command::Term { service }
        | Command::Hup { service }
        | Command::Alarm { service }
        | Command::Interrupt { service }
        | Command::Kill { service } => service,
        _ => unreachable!(),
    };
    let service = Service::new(service_name, config, ServiceSrc::RunSvDir);
    let timeout = Duration::from_secs(cli.timeout);

    service.run(sv_command_type, timeout)
}

// Run the list subcommand
pub fn run_list_command(config: Config, args: ListArgs) -> Result<String, Box<dyn error::Error>> {
    Ok(format_services(
        Service::get_all_services(config)?
            .into_iter()
            .filter(|f| match f.read_status() {
                Ok(status) => {
                    if args.down && status.state != ServiceState::Down {
                        return false;
                    }

                    if args.enabled && f.src == ServiceSrc::ServiceDir {
                        return false;
                    }

                    if args.disabled && f.src == ServiceSrc::RunSvDir {
                        return false;
                    }

                    if args.up && status.state != ServiceState::Run {
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

        if let Err(err) = status {
            return format!("{}", err);
        }

        s.push_str(item.format_status(status.unwrap()).as_str());
    }

    s
}
