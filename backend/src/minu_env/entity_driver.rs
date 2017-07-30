//! Defines the behavior of the universe's entities

use minutiae::prelude::*;
use uuid::Uuid;

use super::*;

pub fn entity_driver(
    universe_index: usize,
    entity: &Entity<CS, ES, MES>,
    entities: &EntityContainer<CS, ES, MES>,
    cells: &[Cell<CS>],
    cell_action_executor: &mut FnMut(CA, usize),
    self_action_executor: &mut FnMut(SelfAction<CS, ES, EA>),
    entity_action_executor: &mut FnMut(EA, usize, Uuid)
) {
    // let (cur_x, cur_y) = get_coords(universe_index, UNIVERSE_SIZE);
    match &entity.state {
        &ES::Messenger(ref lines_opt) => {
            match lines_opt {
                &Some(ref lines) => {
                    // println!("IT WORKS: {:?}", lines);
                    // TODO
                },
                &None => {},
            }

            // reset the contents of
            self_action_executor(SelfAction::Custom(EA::ClearMessengerState));
        },
        _ => unreachable!(),
    }
}
