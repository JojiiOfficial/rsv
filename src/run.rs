use std::error;
use std::time::Duration;

use crate::config::Config;
use crate::sv::cmdtype::SvCommandType;
use crate::sv::service::{Service, ServiceSrc};
use crate::sv::status::ServiceState;

use clap::ArgMatches;

// Run the app
pub fn run(app: &ArgMatches) -> Result<String, Box<dyn error::Error>> {
    let config = Config::new()?;

    // Get current subcommand
    let (subcommand, matches) = app
        .subcommand()
        .ok_or_else(|| "No subcommand provided".to_owned())?;

    #[cfg(feature = "auto_sudo")]
    sudo::escalate_if_needed()?;

    if subcommand == "list" {
        return run_list_command(config, matches);
    }

    // New service from App arg
    let service = Service::new(
        matches
            .value_of("service")
            .ok_or("Service arg missing")?
            .to_owned(),
        config,
        ServiceSrc::RunSvDir,
    );

    // Run the actual command
    service.run(
        SvCommandType::from(subcommand),
        Duration::from_secs(app.value_of("timeout").unwrap_or("7").parse::<u64>()?),
    )
}

// Run the list subcommand
pub fn run_list_command(
    config: Config,
    matches: &ArgMatches,
) -> Result<String, Box<dyn error::Error>> {
    Ok(format_services(
        Service::get_all_services(config)?
            .into_iter()
            .filter(|f| match f.read_status() {
                Ok(status) => {
                    if matches.is_present("down") && status.state != ServiceState::Down {
                        return false;
                    }

                    if matches.is_present("enabled") && f.src == ServiceSrc::ServiceDir {
                        return false;
                    }

                    if matches.is_present("disabled") && f.src == ServiceSrc::RunSvDir {
                        return false;
                    }

                    if matches.is_present("up") && status.state != ServiceState::Run {
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
