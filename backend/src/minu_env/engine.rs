//! Defines functions that define the behavior of the engine.

use minutiae::prelude::{Universe, OwnedAction};

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

fn exec_cell_action(
    action: &OwnedAction<CS, ES, CA, EA>,
    universe: &mut Universe<CS, ES, MES, CA, EA>
) {
    unimplemented!(); // TODO
}

fn exec_self_action(
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

fn exec_entity_action(
    action: &OwnedAction<CS, ES, CA, EA>,
    universe: &mut Universe<CS, ES, MES, CA, EA>
) {
    unimplemented!(); // TODO
}
