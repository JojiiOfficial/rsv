use crate::args;
use std::process;
use sv::cmdtype::SvCommandType;
use sv::service::Service;

// Run the app
pub fn run(opts: args::AppArgs) {
    let (service, cmd_type) = parse_subcommands(opts.cmd);
    run_service_command(service, cmd_type);
}

// Run desired service
fn run_service_command(service: Service, cmd_type: SvCommandType) {
    println!("{}", service.run(cmd_type));
}

// parse the subcommands
fn parse_subcommands(cmds: args::Subcommands) -> (Service, SvCommandType) {
    let (action, sv_type) = match cmds {
        args::Subcommands::Enable(action) => (action, SvCommandType::Enable),
        args::Subcommands::Disable(action) => (action, SvCommandType::Disable),
        args::Subcommands::Start(action) => (action, SvCommandType::Up),
        args::Subcommands::Stop(action) => (action, SvCommandType::Down),
        args::Subcommands::Status(action) => (action, SvCommandType::Status),
        _ => {
            println!("not yet implemented");
            process::exit(1);
        }
    };

    return (action_to_service(action), sv_type);
}

// Get service by action
fn action_to_service(action: args::ServiceAction) -> Service {
    match Service::new(action.service) {
        Ok(service) => service,
        Err(err) => {
            eprint!("Error: {}", err.string());
            process::exit(1);
        }
    }
}
