//! Minutiae client implementation used to display the visualizaion to the user.

extern crate minutiae;
extern crate shared;

use minutiae::prelude::*;
use minutiae::emscripten::{CanvasRenderer, EmscriptenDriver};
use minutiae::engine::serial::SerialEngine;
use minutiae::engine::iterator::{SerialEntityIterator, SerialGridIterator};
use shared::*;
use shared::engine::{exec_actions, exec_self_action, exec_cell_action, exec_entity_action};

extern {
   fn canvas_render(ptr: *const u8);
}

pub fn calc_color(
    cell: &Cell<CS>,
    entity_indexes: &[usize],
    _: &EntityContainer<CS, ES, MES>
) -> [u8; 4] {
    if !entity_indexes.is_empty() { [255, 255, 255, 1] } else {
        match cell.state.highlight_color {
            None => [0, 0, 0, 1],
            Some(QueryType::Select) => [12, 13, 222, 255],
            Some(QueryType::Insert) => [21, 246, 42, 255],
            Some(QueryType::Delete) => [249, 2, 13, 255],
            Some(QueryType::Update) => [212, 11, 212, 255],
            Some(QueryType::Setting) => [72, 72, 72, 255],
            Some(QueryType::Transaction) => [42, 42, 42, 255],
            Some(QueryType::Other) => [59, 183, 212, 255],
        }
    }
}

struct OurEngine;
impl SerialEngine<CS, ES, MES, CA, EA, SerialGridIterator, SerialEntityIterator<CS, ES>> for OurEngine {
    fn iter_cells(&self, cells: &[Cell<CS>]) -> SerialGridIterator {
        SerialGridIterator::new(UNIVERSE_SIZE)
    }

    fn iter_entities(&self, entities: &[Vec<Entity<CS, ES, MES>>]) -> SerialEntityIterator<CS, ES> {
        SerialEntityIterator::new(UNIVERSE_SIZE)
    }

    fn exec_actions(
        &self, mut universe: &mut Universe<CS, ES, MES, CA, EA>, cell_actions: &[OwnedAction<CS, ES, CA, EA>],
        self_actions: &[OwnedAction<CS, ES, CA, EA>], entity_actions: &[OwnedAction<CS, ES, CA, EA>]
    ) {
        for cell_action in cell_actions { exec_cell_action(cell_action, &mut universe); }
        for self_action in self_actions { exec_self_action(self_action, universe); }
        for entity_action in entity_actions { exec_entity_action(entity_action, universe); }
    }
}

fn main() {
    // initialize the minutiae sim
    let conf = UniverseConf {
        iter_cells: false,
        size: UNIVERSE_SIZE,
        view_distance: 1,
    };
    let universe = Universe::new(conf, &mut WG, cell_mutator, entity_driver);
    let driver = EmscriptenDriver;
    let engine: Box<SerialEngine<CS, ES, MES, CA, EA, SerialGridIterator, SerialEntityIterator<CS, ES>>> = Box::new(OurEngine);

    // block on the simulation, looping and processing incoming log lines for the life of the application
    driver.init(universe, engine, &mut [
        Box::new(CanvasRenderer::new(UNIVERSE_SIZE, calc_color, canvas_render))
    ]);
}
