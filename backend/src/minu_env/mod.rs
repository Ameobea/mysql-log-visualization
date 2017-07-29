//! Sets up the environment for the minutiae simulation.  Creates all the necessary state types and creates
//! functions for creating, initializing, and interacting with that environment.

use std::sync::mpsc::Receiver;

use minutiae::prelude::*;
use minutiae::engine::iterator::SerialGridIterator;
use minutiae::engine::parallel::ParallelEngine;

use parser::{parse_line, LogLine};

pub mod engine;
pub mod entity_driver;
pub use self::entity_driver::entity_driver;

pub const UNIVERSE_SIZE: usize = 800;

#[derive(Clone, Debug)]
pub struct CS {}
impl CellState for CS {}

#[derive(Clone, Debug)]
pub struct ES {}
impl EntityState<CS> for ES {}

#[derive(Clone, Copy, Debug, Default)]
pub struct MES {}
impl MutEntityState for MES {}

#[derive(Debug)]
pub enum CA {}

impl CellAction<CS> for CA {}

#[derive(Debug)]
pub enum EA {}
impl EntityAction<CS, ES> for EA {}

pub struct WG;
impl Generator<CS, ES, MES, CA, EA> for WG {
    fn gen(&mut self, conf: &UniverseConf) -> (Vec<Cell<CS>>, Vec<Vec<Entity<CS, ES, MES>>>) {
        // create a blank universe to start off with
        ( vec![Cell{ state: CS {} }; 800 * 800], Vec::new() )
    }
}

// dummy function until `cell_mutator` is deprecated entirely
pub fn cell_mutator(_: usize, _: &[Cell<CS>]) -> Option<CS> { None }

/// Middleware that processes
pub struct LineProcessorMiddleware {
    rx: Receiver<String>,
}

impl LineProcessorMiddleware {
    pub fn new(rx: Receiver<String>) -> Self { LineProcessorMiddleware { rx } }
}

impl Middleware<
    CS, ES, MES, CA, EA, Box<ParallelEngine<CS, ES, MES, CA, EA, SerialGridIterator>>
> for LineProcessorMiddleware {
    fn before_render(&mut self, universe: &mut Universe<CS, ES, MES, CA, EA>) {
        // collect all the lines from stdin into a buffer and process them
        let lines: Vec<String> = self.rx.try_iter().collect();
        // try to parse those lines
        let parsed_lines: Vec<LogLine> = lines.into_iter()
            .fold(Vec::new(), |mut acc, s| {
                match parse_line(&s) {
                    Ok(parsed) => {
                        println!("PARSED LINE: {:?}", parsed);
                        acc.push(parsed);
                        acc
                    },
                    Err(err) => {
                        println!("Error parsing log line: {}", err);
                        acc
                    },
                }
            });
    }
}
