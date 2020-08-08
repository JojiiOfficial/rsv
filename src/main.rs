mod args;
mod run;

fn main() {
    let opt = args::AppArgs::parse();
    run::run(opt);
}
