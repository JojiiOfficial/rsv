use crate::args;
use crate::disable;
use crate::enable;
use crate::start;
use crate::stop;

pub fn run(opts: args::AppArgs) {
    match opts.cmd {
        args::Subcommands::Enable(action) => enable::enable(action),
        args::Subcommands::Disable(action) => disable::disable(action),
        args::Subcommands::Start(action) => start::start(action),
        args::Subcommands::Stop(action) => stop::stop(action),

        _ => return,
    }
}
