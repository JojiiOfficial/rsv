mod args;
mod disable;
mod enable;
mod run;
mod start;
mod stop;

fn main() {
    let opt = args::AppArgs::parse();
    run::run(opt);
}
