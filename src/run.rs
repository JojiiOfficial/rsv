use std::error;
use std::time::Duration;

use crate::config::{self,Config};
use crate::sv::cmdtype::SvCommandType;
use crate::sv::service::{Service, ServiceSrc};
use crate::sv::status::ServiceState;
use crate::sv::error::Error;

use clap::ArgMatches;

// Run the app
pub fn run(app: &ArgMatches) -> Result<String, Box<dyn error::Error>> {

    // Get current subcommand
    let (subcommand, matches) = app
        .subcommand()
        .ok_or_else(|| "No subcommand provided".to_owned())?;

    if subcommand == "init" {
        return config::init_config(matches.is_present("overwrite"));
    }

    let config = config::get()?;

    if subcommand == "list" {
        return run_list_command(config, matches);
    }

    let service_name = matches.value_of("service")
        .ok_or("Service arg missing")?
        .to_owned();

    let service_list: Vec<Service> = Service::get_all_services(config)?;
    
    match  service_list.into_iter().find(|sn| sn.uri == service_name) {
        Some(service) => { 
            // Run the actual command
            return service.run(
                SvCommandType::from(subcommand),
                Duration::from_secs(app.value_of("timeout").unwrap_or("7").parse::<u64>()?),
            )
        }
        None => { // if service not found return error
            return Err(Box::new(Error::ServiceNotFound(service_name.into())))
        }
    }

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
