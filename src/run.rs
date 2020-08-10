use std::env;
use std::error;
use std::path::Path;
use std::process;
use std::time::Duration;

use crate::args::{AppArgs, Subcommands};

use config::conf;
use sv::cmdtype::SvCommandType;
use sv::service::Service;
use sysinfo::SystemExt;

// Run the app
pub fn run(opts: AppArgs) -> Result<String, Box<dyn error::Error>> {
    let mut settings = conf::Settings::new()?;
    init_svdir(&mut settings)?;
    let (service, cmd_type) = parse_subcommands(opts.cmd, settings);

    // Run the actual command
    service.run(cmd_type, Duration::from_secs(opts.timeout.unwrap_or(7)))
}

// parse the subcommands
fn parse_subcommands(cmds: Subcommands, settings: conf::Settings) -> (Service, SvCommandType) {
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

    return (Service::new(action.service, settings), sv_type);
}

fn init_svdir(settings: &mut conf::Settings) -> Result<(), Box<dyn error::Error>> {
    // Check environment variable first
    if let Ok(var) = env::var("SVDIR") {
        if var.len() > 0 {
            settings.runsv_dir = var;
            return Ok(());
        }
    }

    // Only use config if usable
    if settings.runsv_dir.len() > 1 && Path::new(&settings.runsv_dir.as_str()).exists() {
        return Ok(());
    }

    let sys = sysinfo::System::new();
    let mut was_p = false;

    for (_, v) in sys.get_process_list().iter() {
        if !v.name.contains("runsvdir") {
            continue;
        }

        for arg in v.cmd.iter() {
            if arg == "-P" {
                was_p = true;
                continue;
            }

            if was_p && arg.len() > 0 && arg.starts_with("/") {
                settings.runsv_dir = arg.clone();
                return settings.save();
            }
        }
    }

    Ok(())
}
