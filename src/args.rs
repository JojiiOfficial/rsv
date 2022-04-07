use clap::{AppSettings, Args, Parser, Subcommand};
use clap_complete::Shell;

/// A tool to maintain runit services like systemd services
#[derive(Debug, Parser)]
#[clap(version, author = "Jojii S")]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
pub struct Cli {
    /// Print more verbose info
    #[clap(short, long, global = true)]
    pub verbose: bool,

    /// Timeout for commands in secs
    #[clap(short, long, global = true, default_value_t = 7)]
    pub timeout: u64,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Enable a service
    Enable {
        /// Service to be enabled
        service: String,
    },

    /// Disable a service
    Disable {
        /// Service to be disabled
        service: String,
    },

    /// Start a service
    Start {
        /// Service to be started
        service: String,
    },

    /// Stop a service
    Stop {
        /// Service to be stopped
        service: String,
    },

    /// Restart a service
    Restart {
        /// Service to be restarted
        service: String,
    },

    /// Get status of a service
    Status {
        /// Service to get status of
        service: String,
    },

    /// Start if service is not running. Do not restart if it stops.
    Once {
        /// Service to be started once
        service: String,
    },

    /// Send SIGSTOP if the service is running
    Pause {
        /// Service to be paused
        service: String,
    },

    /// Send SIGCONT if the service is running
    Continue {
        /// Service to be continued
        service: String,
    },

    /// Send SIGTERM if the service is running
    Term {
        /// Service to be terminated
        service: String,
    },

    /// Send SIGHUP if the service is running
    Hup {
        /// Service to be hanged up
        service: String,
    },

    /// Send SIGALARM if the service is running
    Alarm {
        /// Service to send the SIGALARM signal to
        service: String,
    },

    /// Send SIGINT if the service is running
    Interrupt {
        /// Service to be interrupted
        service: String,
    },

    /// Send SIGKILL if the service is running
    Kill {
        /// Service to be killed
        service: String,
    },

    /// List services
    List(ListArgs),

    /// Generate completion script for a given shell
    Completion {
        /// Shell to generate completion for
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Args, Debug)]
pub struct ListArgs {
    #[clap(short, long)]
    pub all: bool,

    /// List only services that are up
    #[clap(short, long, group = "up_down")]
    pub up: bool,

    /// List only services that are down
    #[clap(short = 'd', long, group = "up_down")]
    pub down: bool,

    /// List only services that are enabled
    #[clap(short, long, group = "enabled_disabled")]
    pub enabled: bool,

    /// List only services that are disabled
    #[clap(short = 'D', long, group = "enabled_disabled")]
    pub disabled: bool,
}
