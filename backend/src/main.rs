//! MySQL Server Log File Visualization Backend.  Provides the log tailing functionality as well as hosting the
//! Minutiae server to power the visualization and transmit events to the clients.

#![feature(try_from)]

extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate minutiae;
extern crate regex;
extern crate uuid;

use minutiae::prelude::*;
use minutiae::driver::BasicDriver;
use minutiae::engine::parallel::ParallelEngine;
use minutiae::engine::iterator::SerialGridIterator;
use minutiae::driver::middleware::MinDelay;

pub mod input_reader;
pub mod minu_env;
use self::minu_env::*;
use self::minu_env::engine::{exec_actions};
pub mod parser;
pub mod util;

fn main() {
    let rx = input_reader::read_lines();

    // set up the minutiae environment
    let conf = UniverseConf {
        iter_cells: false,
        size: UNIVERSE_SIZE,
        view_distance: 1,
    };
    let universe = Universe::new(conf, &mut WG, cell_mutator, entity_driver);
    let driver = BasicDriver;
    let engine = ParallelEngine::new(SerialGridIterator::new(UNIVERSE_SIZE * UNIVERSE_SIZE), exec_actions, entity_driver);

    // block on the simulation, looping and processing incoming log lines for the life of the application
    driver.init(universe, Box::new(engine), &mut [
        Box::new(LineProcessorMiddleware::new(rx)),
        Box::new(MinDelay::from_tps(59.97)),
    ]);
}
