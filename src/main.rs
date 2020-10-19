mod args;
mod config;
mod run;
mod sv;

fn main() {
    match run::run(args::get_cli()) {
        Ok(s) => print!("{}", s),
        Err(e) => eprintln!("An error occured: {}", e),
    }
}
