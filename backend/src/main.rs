//! MySQL Server Log File Visualization Backend.  Provides the log tailing functionality as well as hosting the
//! Minutiae server to power the visualization and transmit events to the clients.

#![feature(try_from)]

extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate minutiae;
extern crate regex;

pub mod input_reader;
pub mod minu_env;
pub mod parser;
pub mod util;

fn main() {
    let rx = input_reader::read_lines();

    let universe = minu_env::create_universe();
}
