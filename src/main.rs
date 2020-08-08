mod args;
mod run;
use sv;

fn main() {
    let opt = args::AppArgs::parse();
    run::run(opt);
}
