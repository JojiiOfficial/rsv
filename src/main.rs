mod args;
mod config;
mod run;
mod sv;

use args::{Cli, Command};
use clap::{crate_name, IntoApp, Parser};
use clap_complete::Shell;

fn main() {
    let cli = Cli::parse();

    // Generate completion
    if let Command::Completion { shell } = cli.command {
        generate_completion(shell);
        return;
    }

    match run::run(cli) {
        Ok(s) => print!("{}", s),
        Err(e) => eprintln!("An error occured: {}", e),
    }
}

fn generate_completion(shell: Shell) {
    clap_complete::generate(
        shell,
        &mut Cli::command(),
        crate_name!(),
        &mut std::io::stdout(),
    )
}
