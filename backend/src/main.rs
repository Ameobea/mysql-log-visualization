//! MySQL Server Log File Visualization Backend.  Provides the log tailing functionality as well as hosting the
//! Minutiae server to power the visualization and transmit events to the clients.

#![feature(conservative_impl_trait, try_from, unboxed_closures)]

extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate minutiae;
extern crate pcg;
extern crate rand;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate shared;
extern crate uuid;

use minutiae::prelude::*;
use minutiae::driver::BasicDriver;
use minutiae::engine::parallel::{ActionExecutor, ParallelEngine};
use minutiae::engine::iterator::SerialGridIterator;
use minutiae::driver::middleware::MinDelay;
use minutiae::server::{Server, HybridServer};
use shared::UNIVERSE_SIZE;

pub mod input_reader;
pub mod minu_env;
use self::minu_env::*;
pub mod parser;
pub mod util;

/// This function is responsible for taking in the three action buffers and determining what to actually send to the
/// client.  As a result, it produces the actual events that are transmitted to the client.
fn event_generator_generator() -> impl Fn(
    &mut Universe<CS, ES, MES, CA, EA>, &[OwnedAction<CS, ES, CA, EA>],
    &[OwnedAction<CS, ES, CA, EA>], &[OwnedAction<CS, ES, CA, EA>]
) -> Option<Vec<OurEvent>> {
    |
        universe: &mut Universe<CS, ES, MES, CA, EA>, self_actions: &[OwnedAction<CS, ES, CA, EA>],
        cell_actions: &[OwnedAction<CS, ES, CA, EA>], entity_actions: &[OwnedAction<CS, ES, CA, EA>]
    | {
        // unimplemented!(); // TODO
        None
    }
}

// dummy function until `cell_mutator` is deprecated entirely
pub fn cell_mutator(_: usize, _: &[Cell<CS>]) -> Option<CS> { None }

fn main() {
    let rx = input_reader::read_lines();

    // construct the hybrid server that sends events to the clients
    let event_generator = event_generator_generator();
    let (hooked_action_executor, server_logic): (ActionExecutor<CS, ES, MES, CA, EA>, _) = HybridServer::hook_handler(exec_actions, event_generator);
    let seq = server_logic.seq.clone();
    let server = Box::new(Server::new(UNIVERSE_SIZE, "localhost:3012", server_logic, seq));

    // set up the minutiae environment
    let conf = UniverseConf {
        iter_cells: false,
        size: UNIVERSE_SIZE,
        view_distance: 1,
    };
    let universe = Universe::new(conf, &mut WG, cell_mutator, entity_driver);
    let driver = BasicDriver;
    let engine = ParallelEngine::new(SerialGridIterator::new(UNIVERSE_SIZE * UNIVERSE_SIZE), hooked_action_executor, entity_driver);

    // block on the simulation, looping and processing incoming log lines for the life of the application
    driver.init(universe, Box::new(engine), &mut [
        Box::new(LineProcessorMiddleware::new(rx)),
        Box::new(MinDelay::from_tps(59.97)),
        server, // Very impressed with myself at being able to make this a middleware tbh
    ]);
}
