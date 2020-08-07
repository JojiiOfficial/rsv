use crate::{args::ServiceAction, sv};

pub fn status(opts: ServiceAction) {
    sv::print_output(sv::status(opts.service.as_str()))
}
