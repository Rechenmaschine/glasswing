use std::fmt::Debug;
use crate::agents::{sort_actions, Evaluator, SymmetricEvaluation};
use crate::core::{Game, GwState};
use num_traits::Bounded;
use std::marker::PhantomData;
use std::ops::Neg;

pub struct NegaMax<G, E>
where
    G: Game,
    G::EvalType: Ord + Bounded + Neg<Output=G::EvalType> + Copy,
    E: Evaluator<G> + SymmetricEvaluation<G>,
{
    depth: u32,
    evaluator: E,
    _game: PhantomData<G>,
}

impl<G, E> NegaMax<G, E>
    where
        G: Game,
        G::EvalType: Ord + Bounded + Neg<Output=G::EvalType> + Copy,
        E: Evaluator<G> + SymmetricEvaluation<G>,
{
    pub fn new(depth: u32, evaluator: E) -> Self {
        NegaMax {
            depth,
            evaluator,
            _game: PhantomData,
        }
    }

    /*
    function negamax(node, depth, α, β) is
    if depth = 0 or node is a terminal node then
        return evaluation of node

    childNodes := generateMoves(node)
    childNodes := orderMoves(childNodes)
    value := −∞
    foreach child in childNodes do
        value := max(value, −negamax(child, depth − 1, −β, −α))
        α := max(α, value)
        if α ≥ β then
            break (* cut-off *)
    return value
     */

    pub fn negamax(
        &mut self,
        state: &G::State,
        depth: u32,
        mut alpha: G::EvalType,
        beta: G::EvalType,
    ) -> G::EvalType {
        // In most games we hit the depth limit before we hit a terminal state,
        // therefore it is more efficient to check for the depth limit first.
        if depth == 0 || state.is_terminal() {
            return self.evaluator.evaluate_for(state, &state.team_to_move());
        }

        // Generate all legal actions from the current state and sort in ascending order of heuristic.
        let mut actions = state.actions().into_iter().collect::<Vec<G::Action>>();
        sort_actions(state, &mut actions, &mut self.evaluator, &state.team_to_move());

        // iterate in descending order as per negamax optimisation
        let mut value = -G::EvalType::max_value();
        for action in actions.iter().rev() {
            let new_state = state.apply_action(&action);
            let eval = -self.negamax(&new_state, depth - 1, -beta, -alpha);
            value = value.max(eval);
            alpha = alpha.max(value);
            if alpha >= beta {
                break; // (* cut-off *)
            }
        }
        value
    }
}

impl<G, E> Evaluator<G> for NegaMax<G, E>
    where
        G: Game,
        G::EvalType: Ord + Bounded + Neg<Output=G::EvalType> + Copy,
        E: Evaluator<G> + SymmetricEvaluation<G>,
{
    fn evaluate_for(&mut self, state: &G::State, for_team: &G::Team) -> G::EvalType {
        if state.team_to_move() == *for_team {
            // Hacky workaround to avoid overflow. TODO fix properly.
            self.negamax(state, self.depth, -G::EvalType::max_value(), G::EvalType::max_value())
        }else {
            -self.negamax(state, self.depth, -G::EvalType::max_value(), G::EvalType::max_value())
        }
    }
}

impl<G, E> SymmetricEvaluation<G> for NegaMax<G, E>
    where
        G: Game,
        G::EvalType: Ord + Bounded + Neg<Output=G::EvalType> + Copy + Debug,
        E: Evaluator<G> + SymmetricEvaluation<G>,
{}
