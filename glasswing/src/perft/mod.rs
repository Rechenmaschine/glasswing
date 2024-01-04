use crate::core::{Game, GwState};

pub fn perft<G: Game>(state: &G::State, depth: u32) -> usize {
    if state.is_terminal() {
        return 1;
    } else if depth == 1 {
        return state.count_actions();
    }

    let count = state
        .actions()
        .into_iter()
        .map(|action| {
            let new_state = state.apply_action(&action);
            perft::<G>(&new_state, depth - 1)
        })
        .sum::<usize>();

    count
}
