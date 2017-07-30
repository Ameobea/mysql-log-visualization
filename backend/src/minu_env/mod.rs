//! Sets up the environment for the minutiae simulation.  Creates all the necessary state types and creates
//! functions for creating, initializing, and interacting with that environment.

use std::sync::mpsc::Receiver;

use minutiae::prelude::*;
use minutiae::engine::iterator::SerialGridIterator;
use minutiae::engine::parallel::ParallelEngine;
use minutiae::server::HybParam;
use minutiae::server::Event;
use pcg::PcgRng;

use parser::parse_line;
// use shared::*;
use shared::{LogLine, QueryType};

pub mod engine;
pub use engine::exec_actions;
pub mod entity_driver;
pub use entity_driver::entity_driver;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CS {
    pub highlight_color: Option<QueryType>,
}
impl CellState for CS {}
impl HybParam for CS {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ES {
    Messenger(Option<Vec<LogLine>>),
    Normal,
}
impl EntityState<CS> for ES {}
impl HybParam for ES {}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MES {
    rng: PcgRng,
}
impl Default for MES { fn default() -> Self { MES { rng: PcgRng::new_unseeded() } } }
impl MutEntityState for MES {}
impl HybParam for MES {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CA {
    HilightCells(Option<QueryType>, PcgRng),
}
impl HybParam for CA {}

impl CellAction<CS> for CA {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EA {
    ClearMessengerState,
}
impl EntityAction<CS, ES> for EA {}
impl HybParam for EA {}

pub struct WG;
impl Generator<CS, ES, MES, CA, EA> for WG {
    fn gen(&mut self, _: &UniverseConf) -> (Vec<Cell<CS>>, Vec<Vec<Entity<CS, ES, MES>>>) {
        // create a blank universe to start off with
        (
            vec![Cell{ state: CS { highlight_color: None } }; 800 * 800],
            vec![vec![Entity::new(ES::Messenger(None), MES::default())]]
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OurEvent {

}

impl Event<CS, ES, MES, CA, EA> for OurEvent {
    /// This is used on the client side to actually apply the events to the server.
    fn apply(&self, universe: &mut Universe<CS, ES, MES, CA, EA>) {
        unimplemented!(); // TODO
    }
}
impl HybParam for OurEvent {}

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

        if parsed_lines.len() > 0 {
            // set them into the state of the messenger entity
            let messenger_entity = unsafe { universe.entities.get_mut(0) };
            messenger_entity.state = ES::Messenger(Some(parsed_lines));
        }
    }
}
