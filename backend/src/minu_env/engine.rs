//! Defines functions that define the behavior of the engine.

use super::*;

pub fn exec_actions(
    universe: &mut Universe<CS, ES, MES, CA, EA>,
    self_actions: &[OwnedAction<CS, ES, CA, EA>],
    cell_actions: &[OwnedAction<CS, ES, CA, EA>],
    entity_actions: &[OwnedAction<CS, ES, CA, EA>]
) {
    unimplemented!(); // TODO
}
