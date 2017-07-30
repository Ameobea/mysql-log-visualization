//! Defines functions that define the behavior of the engine.

use minutiae::prelude::{CellState, EntityState, MutEntityState, CellAction, EntityAction, Universe, OwnedAction};
use minutiae::server::HybParam;

pub fn exec_actions<
    C: CellState, E: EntityState<C>, M: MutEntityState,
    CA: CellAction<C>, EA: EntityAction<C, E>>
(
    universe: &mut Universe<C, E, M, CA, EA>,
    self_actions: &[OwnedAction<C, E, CA, EA>],
    cell_actions: &[OwnedAction<C, E, CA, EA>],
    entity_actions: &[OwnedAction<C, E, CA, EA>]
) {
    // unimplemented!(); // TODO
}
