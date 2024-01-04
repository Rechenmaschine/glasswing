use crate::core::{Game, GwState};

pub fn perft<G: Game>(state: &G::State, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    perft_recursive::<G>(state, depth)
}

fn perft_recursive<G: Game>(state: &G::State, depth: u32) -> u64 {
    if state.is_terminal() {
        return 1;
    } else if depth == 1 {
        return state.count_actions() as u64;
    }

    let count = state
        .actions()
        .into_iter()
        .map(|action| {
            let new_state = state.apply_action(&action);
            perft_recursive::<G>(&new_state, depth - 1)
        })
        .sum::<u64>();

    count
}
