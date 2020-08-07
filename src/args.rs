use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct AppArgs {
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u32,

    #[structopt(subcommand)]
    pub cmd: Subcommands,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "A tool to maintain runit services like systemd services")]
pub enum Subcommands {
    #[structopt(about = "Enable a service")]
    Enable(ServiceAction),

    #[structopt(about = "Disable a service")]
    Disable(ServiceAction),

    #[structopt(about = "Start a service")]
    Start(ServiceAction),

    #[structopt(about = "Stop a service")]
    Stop(ServiceAction),

    #[structopt(about = "Status a service")]
    Status(ServiceAction),
}

#[derive(StructOpt, Debug)]
pub struct ServiceAction {
    pub service: String,
}

impl AppArgs {
    pub fn parse() -> AppArgs {
        let args = AppArgs::from_args();
        return args;
    }
}