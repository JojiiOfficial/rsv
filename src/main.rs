mod args;
mod conf;
mod run;

fn main() {
    let settings = match conf::Settings::new() {
        Ok(s) => s,
        Err(v) => {
            eprintln!("Error reading config: {}", v);
            return;
        }
    };

    let opt = args::AppArgs::parse();
    run::run(opt);
}
