mod transposition;

use std::ops::Range;
use std::time::Instant;
use std::time::Duration;
use std::fmt;
use std::hash::Hash;
use transposition::TranspositionTable;
use crate::core::{Game, State};

pub struct PerftResult {
    depth: u32,
    nodes: usize,
    time: Duration,
}

impl PerftResult {
    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn nodes(&self) -> usize {
        self.nodes
    }

    pub fn time(&self) -> Duration {
        self.time
    }

    pub fn nps(&self) -> Nps {
        Nps(self.nodes, self.time)
    }
}


pub struct Nps(usize, Duration);

impl Nps {
    pub fn as_f64(&self) -> f64 {
        self.0 as f64 / self.1.as_secs_f64()
    }
}

impl fmt::Display for Nps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nodes_per_sec = self.as_f64();
        if nodes_per_sec < 1_000.0 {
            return write!(f, "{:.2} n/s", nodes_per_sec);
        }
        let kn_per_sec = nodes_per_sec / 1_000.0;
        if kn_per_sec < 1_000.0 {
            return write!(f, "{:.2} Kn/s", kn_per_sec);
        }
        let mn_per_sec = kn_per_sec / 1_000.0;
        if mn_per_sec < 1_000.0 {
            return write!(f, "{:.2} Mn/s", mn_per_sec);
        }
        let gn_per_sec = mn_per_sec / 1_000.0;
        if gn_per_sec < 1_000.0 {
            return write!(f, "{:.2} Gn/s", gn_per_sec);
        }
        let tn_per_sec = gn_per_sec / 1_000.0;
        if tn_per_sec < 1_000.0 {
            return write!(f, "{:.2} Tn/s", tn_per_sec);
        }
        let pn_per_sec = tn_per_sec / 1_000.0;
        if pn_per_sec < 1_000.0 {
            return write!(f, "{:.2} Pn/s", pn_per_sec);
        }
        let en_per_sec = pn_per_sec / 1_000.0;
        write!(f, "{:.2} En/s", en_per_sec)
    }
}

pub fn incremental_perft<G: Game, F: Fn(u32) -> u32>(state: &G::State, range: Range<u32>, table_depth: F) -> Vec<PerftResult>
    where G::State: Hash + Eq {
    let mut vec = vec![];
    for i in range {
        let tr_depth = table_depth(i);
        let res = perft::<G>(&state, i, tr_depth);
        vec.push(res);
    }
    vec
}

pub fn perft<G: Game>(state: &G::State, depth: u32, tr_depth: u32) -> PerftResult
    where G::State: Hash + Eq
{
    if depth == 0 {
        PerftResult {
            depth: 0,
            nodes: 1,
            time: Duration::ZERO,
        }
    } else {
        let mut tr = TranspositionTable::<G>::new();
        let start_time = Instant::now();
        let nodes = perft_recursive::<G>(&mut tr, state, depth - 1, tr_depth);
        let elapsed = start_time.elapsed();
        PerftResult {
            depth,
            nodes,
            time: elapsed,
        }
    }
}

fn perft_recursive<G: Game>(tr: &mut TranspositionTable<G>, state: &G::State, depth: u32, tr_depth: u32) -> usize
    where G::State: Hash + Eq
{
    if state.is_terminal() {
        return 1;
    }

    if depth == 0 {
        return state.count_actions();
    }

    let mut nodes = 0;
    let actions = state.actions();

    for action in actions {
        let new_state = state.apply_action(&action);

        let child_nodes = if tr_depth == 0 { // then dont use TR
            perft_recursive::<G>(tr, &new_state, depth - 1, 0)
        } else {
            if let Some(child_nodes) = tr.get(&new_state) {
                child_nodes
            } else {
                let child_nodes = perft_recursive::<G>(tr, &new_state, depth - 1, tr_depth - 1);
                tr.insert(new_state, child_nodes);
                child_nodes
            }
        };
        nodes += child_nodes;
    }

    nodes
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}