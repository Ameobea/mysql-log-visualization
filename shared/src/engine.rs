//! Defines functions that define the behavior of the engine.

use minutiae::prelude::{Universe, OwnedAction};
use rand::Rng;

use super::*;

pub fn exec_actions(
    universe: &mut Universe<CS, ES, MES, CA, EA>,
    self_actions: &[OwnedAction<CS, ES, CA, EA>],
    cell_actions: &[OwnedAction<CS, ES, CA, EA>],
    entity_actions: &[OwnedAction<CS, ES, CA, EA>]
) {
    // process actions in order of cell actions, then self actions, and finally entity actions
    for cell_action in cell_actions {
        exec_cell_action(cell_action, universe);
    }

    for self_action in self_actions {
        exec_self_action(self_action, universe);
    }

    for entity_action in entity_actions {
        exec_entity_action(entity_action, universe);
    }
}

pub fn exec_cell_action(
    action: &OwnedAction<CS, ES, CA, EA>,
    universe: &mut Universe<CS, ES, MES, CA, EA>
) {
    match &action.action {
        &Action::CellAction { action: ref inner_action, universe_index } => {
            match inner_action {
                &CA::HilightCells(query_type_opt, mut rng) => {
                    let (mut x, mut y) = get_coords(universe_index, UNIVERSE_SIZE);
                    // iterate over the selected coordinate until we've set the state for all cells
                    let mut swapped_entities = 0;
                    loop {
                        if swapped_entities >= QUERY_ENTITY_COUNT {
                            break;
                        } else {
                            let (x_offset, y_offset) = (rng.gen_range(-1, 2), rng.gen_range(-1, 2));
                            let (proposed_x, proposed_y) = (x as isize + x_offset, y as isize + y_offset);
                            if proposed_x >= 0 && proposed_x < (UNIVERSE_SIZE * UNIVERSE_SIZE) as isize &&
                               proposed_y >= 0 && proposed_y < (UNIVERSE_SIZE * UNIVERSE_SIZE) as isize {
                                x = proposed_x as usize;
                                y = proposed_y as usize;
                                if universe.cells[get_index(x, y, UNIVERSE_SIZE)].state.highlight_color.is_none() {
                                    universe.cells[get_index(x, y, UNIVERSE_SIZE)].state.highlight_color = query_type_opt;
                                    swapped_entities += 1;
                                }
                            }
                        }
                    }
                },
            }
        },
        _ => unreachable!(),
    }
}

pub fn exec_self_action(
    action: &OwnedAction<CS, ES, CA, EA>,
    universe: &mut Universe<CS, ES, MES, CA, EA>
) {
    match &action.action {
        &Action::SelfAction(ref inner_action) => {
            match inner_action {
                &SelfAction::Custom(ref ea) => {
                    match ea {
                        &EA::ClearMessengerState => {
                            match universe.entities.get_verify_mut(action.source_entity_index, action.source_uuid) {
                                Some((entity, _)) => {
                                    entity.state = ES::Messenger(None);
                                },
                                None => println!("ERROR: Messenger entity went away while trying to set its state!"),
                            }
                        }
                    }
                },
                _ => unimplemented!(),
            }
        },
        _ => unreachable!(),
    }
}

pub fn exec_entity_action(
    action: &OwnedAction<CS, ES, CA, EA>,
    universe: &mut Universe<CS, ES, MES, CA, EA>
) {
    unimplemented!(); // TODO
}
