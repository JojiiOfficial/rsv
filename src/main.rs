mod args;
mod config;
mod run;
mod sv;

use clap::App;
use clap_generate::{
    generate,
    generators::{Bash, Elvish, Fish, Zsh},
    Generator,
};

fn main() {
    let app = args::get_cli().get_matches();

    // Run generator command if desired
    if let Some(generator) = app.value_of("generator") {
        generate_completions(generator);
        return;
    }

    match run::run(&app) {
        Ok(s) => print!("{}", s),
        Err(e) => eprintln!("An error occured: {}", e),
    }
}

fn generate_completions(generator: &str) {
    let mut app = args::get_cli();
    match generator {
        "bash" => print_completions::<Bash>(&mut app),
        "elvish" => print_completions::<Elvish>(&mut app),
        "fish" => print_completions::<Fish>(&mut app),
        "zsh" => print_completions::<Zsh>(&mut app),
        _ => println!("Unknown generator"),
    }
}

fn print_completions<G: Generator>(app: &mut App) {
    generate::<G, _>(app, app.get_name().to_string(), &mut std::io::stdout());
}
