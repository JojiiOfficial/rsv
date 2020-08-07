use crate::args::ServiceAction;
use crate::sv;

pub fn stop(opts: ServiceAction) {
    sv::print_output(sv::stop(opts.service.as_str()))
}
