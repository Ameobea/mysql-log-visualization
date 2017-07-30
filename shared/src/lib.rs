//! Contains code shared between the client and the server, mostly minutiae stuff since they share the same minutiae
//! environment and pass messages back and forth.

#![feature(try_from)]

extern crate chrono;
extern crate minutiae;
extern crate pcg;
extern crate rand;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use std::convert::TryFrom;

use chrono::NaiveDateTime;
use minutiae::prelude::*;
use pcg::PcgRng;
use regex::Match;

pub mod engine;
pub mod entity_driver;
pub use entity_driver::entity_driver;

pub const UNIVERSE_SIZE: usize = 800;
pub const QUERY_ENTITY_COUNT: usize = 25;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CS {
    pub highlight_color: Option<QueryType>,
}
impl CellState for CS {}
// impl HybParam for CS {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ES {
    Messenger(Option<Vec<LogLine>>),
    Normal,
}
impl EntityState<CS> for ES {}
// impl HybParam for ES {}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MES {
    rng: PcgRng,
}
impl Default for MES { fn default() -> Self { MES { rng: PcgRng::new_unseeded() } } }
impl MutEntityState for MES {}
// impl HybParam for MES {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CA {
    HilightCells(Option<QueryType>, PcgRng),
}
// impl HybParam for CA {}

impl CellAction<CS> for CA {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EA {
    ClearMessengerState,
}
impl EntityAction<CS, ES> for EA {}
// impl HybParam for EA {}

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

// dummy function until `cell_mutator` is deprecated entirely
pub fn cell_mutator(_: usize, _: &[Cell<CS>]) -> Option<CS> { None }

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum QueryType {
    Insert,
    Select,
    Update,
    Delete,
    Other,
    Transaction,
    Setting,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogLine {
    pub date: NaiveDateTime,
    pub event_type: LineType,
    pub query_type: Option<QueryType>,
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum LineType {
    Connect,
    Query,
    Quit,
}

impl<'a> TryFrom<Match<'a>> for LineType {
    type Error = String;

    fn try_from(m: Match<'a>) -> Result<Self, Self::Error> {
        let m_str = m.as_str();

        match m_str {
            "Connect" => Ok(LineType::Connect),
            "Query" => Ok(LineType::Query),
            "Quit" => Ok(LineType::Quit),
            _ => Err(format!("Unable to parse line type of type: {}", m_str)),
        }
    }
}
