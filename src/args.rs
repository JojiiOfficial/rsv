use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct AppArgs {
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u32,

    #[structopt(short, long, global = true)]
    pub timeout: Option<u64>,

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

    #[structopt(about = "Restart a service")]
    Restart(ServiceAction),

    #[structopt(about = "Start if service is not running. Do not restart if it stops")]
    Once(ServiceAction),

    #[structopt(about = "Send SIGSTOP if service is running")]
    Pause(ServiceAction),

    #[structopt(about = "Send SIGCONT if service is running")]
    Continue(ServiceAction),

    #[structopt(about = "Send SIGTERM if service is running")]
    Term(ServiceAction),

    #[structopt(about = "Send SIGHUP if service is running")]
    Hup(ServiceAction),

    #[structopt(about = "Send SIGALARM if service is running")]
    Alarm(ServiceAction),

    #[structopt(about = "Send SIGINT if service is running")]
    Interrupt(ServiceAction),

    #[structopt(about = "Send SIGKILL if service is running")]
    Kill(ServiceAction),
}

#[derive(StructOpt, Debug)]
pub struct ServiceAction {
    pub service: String,
}

impl AppArgs {
    pub fn parse() -> AppArgs {
        AppArgs::from_args()
    }
}
