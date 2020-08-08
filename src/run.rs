use crate::args;
use crate::disable;
use crate::enable;
use crate::sv;

pub fn run(opts: args::AppArgs) {
    match opts.cmd {
        args::Subcommands::Enable(action) => enable::enable(action),
        args::Subcommands::Disable(action) => disable::disable(action),
        args::Subcommands::Start(action) => sv::start(action),
        args::Subcommands::Stop(action) => sv::stop(action),
        args::Subcommands::Status(action) => sv::status(action),
        _ => println!("not yet implemented"),
    }
}
