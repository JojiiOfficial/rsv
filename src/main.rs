mod args;
mod disable;
mod enable;
mod run;
mod sv;

fn main() {
    let opt = args::AppArgs::parse();
    run::run(opt);
}
