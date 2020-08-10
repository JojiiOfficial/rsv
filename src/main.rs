mod args;
mod run;

fn main() {
    let opt = args::AppArgs::parse();

    match run::run(opt) {
        Ok(s) => print!("{}", s),
        Err(e) => eprintln!("An error occured: {}", e),
    }
}
