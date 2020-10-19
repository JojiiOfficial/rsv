use std::error;
use std::time::Duration;

use crate::config::Config;
use crate::sv::cmdtype::SvCommandType;
use crate::sv::service::{Service, ServiceSrc};
use crate::sv::status::ServiceState;

use clap::{App, ArgMatches};

// Run the app
pub fn run(opts: App) -> Result<String, Box<dyn error::Error>> {
    let config = Config::new()?;

    let app = opts.get_matches();

    if let Some(subcommand) = app.subcommand_matches("list") {
        return run_list_command(config, subcommand);
    }

    let (service, cmd_type) = parse_subcommands(config, &app);

    // Run the actual command
    service.run(
        cmd_type,
        Duration::from_secs(app.value_of("timeout").unwrap_or("7").parse::<u64>()?),
    )
}

// Parse the subcommands
fn parse_subcommands(config: Config, matches: &ArgMatches) -> (Service, SvCommandType) {
    let (subcommand, matches) = matches.subcommand().unwrap();

    (
        Service::new(
            matches.value_of("service").unwrap().to_owned(),
            config,
            ServiceSrc::RunSvDir,
        ),
        SvCommandType::from(subcommand),
    )
}

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

        if !status.is_ok() {
            return format!("{}", status.err().unwrap());
        }

        s.push_str(format!("{}", item.format_status(status.unwrap())).as_str());
    }

    s
}
