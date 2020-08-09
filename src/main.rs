mod args;
mod run;
use config::conf;

fn main() {
    let settings = match conf::Settings::new() {
        Ok(s) => s,
        Err(v) => {
            eprintln!("Error reading config: {}", v);
            return;
        }
    };

    let opt = args::AppArgs::parse();

    match run::run(opt, settings) {
        Ok(s) => print!("{}", s),
        Err(e) => eprint!("An error occured: {}", e),
    }
}
