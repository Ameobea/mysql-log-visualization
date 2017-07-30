//! Defines the behavior of the universe's entities

use minutiae::prelude::*;
use rand::Rng;
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
                    // process these new log lines and convert them into actual events
                    for line in lines {
                        // pick a random coordinate on the canvas to use as a base for the event
                        let mut rng = entity.mut_state.take().rng;
                        let (mut x, mut y): (usize, usize) = (
                            rng.gen_range(0, UNIVERSE_SIZE * UNIVERSE_SIZE),
                            rng.gen_range(0, UNIVERSE_SIZE * UNIVERSE_SIZE)
                        );
                        let event_universe_index = rng.gen_range(0, UNIVERSE_SIZE * UNIVERSE_SIZE);

                        cell_action_executor(CA::HilightCells(line.query_type, rng.clone().with_stream(rng.gen())), event_universe_index);

                        entity.mut_state.set(MES { rng });
                    }

                    // reset the contents of
                    self_action_executor(SelfAction::Custom(EA::ClearMessengerState));
                },
                &None => {},
            }
        },
        _ => unreachable!(),
    }
}
