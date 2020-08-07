mod args;
mod disable;
mod enable;
mod run;
mod start;
mod status;
mod stop;
mod sv;

fn main() {
    let opt = args::AppArgs::parse();
    run::run(opt);
}
