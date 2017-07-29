//! Sets up the environment for the minutiae simulation.  Creates all the necessary state types and creates
//! functions for creating, initializing, and interacting with that environment.

use std::sync::mpsc::Receiver;

use minutiae::prelude::*;
use minutiae::engine::serial::SerialEngine;
use minutiae::engine::iterator::{SerialGridIterator, SerialEntityIterator};

use parser::{LogLine, LineType};

const UNIVERSE_SIZE: usize = 800;

#[derive(Clone)]
pub struct CS {}
impl CellState for CS {}

#[derive(Clone)]
pub struct ES {}
impl EntityState<CS> for ES {}

#[derive(Clone, Copy, Default)]
pub struct MES {}
impl MutEntityState for MES {}

pub enum CA {}

impl CellAction<CS> for CA {}

pub enum EA {}
impl EntityAction<CS, ES> for EA {}

pub struct WG;
impl Generator<CS, ES, MES, CA, EA> for WG {
    fn gen(&mut self, conf: &UniverseConf) -> (Vec<Cell<CS>>, Vec<Vec<Entity<CS, ES, MES>>>) {
        // create a blank universe to start off with
        ( vec![Cell{ state: CS {} }; 800 * 800], Vec::new() )
    }
}

pub fn create_universe() -> Universe<CS, ES, MES, CA, EA> {
    unimplemented!(); // TODO
}

/// Middleware that processes
pub struct LineProcessorMiddleware {
    rx: Receiver<String>,
}

impl LineProcessorMiddleware {
    pub fn new(rx: Receiver<String>) -> Self {
        LineProcessorMiddleware { rx }
    }
}

impl Middleware<
    CS, ES, MES, CA, EA, Box<SerialEngine<CS, ES, MES, CA, EA, SerialGridIterator, SerialEntityIterator<CS, ES>>>
> for LineProcessorMiddleware {
    fn before_render(&mut self, universe: &mut Universe<CS, ES, MES, CA, EA>) {
        // collect all the lines from stdin into a buffer and process them
        let lines: Vec<String> = self.rx.try_iter().collect();
        // try to parse those lines

    }
}
