use crate::args::ServiceAction;
use crate::sv;

pub fn start(opts: ServiceAction) {
    sv::print_output(sv::start(opts.service.as_str()))
}
