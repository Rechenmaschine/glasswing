use cachewing::{QuadraticProbingTable64, TranspositionHash, TranspositionTable};
use glasswing::core::{Game, GwState};
use glasswing_games::connect4::Connect4;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <depth> <log2(slots)>", args[0]);
        return;
    }

    let depth = args[1].parse::<usize>().expect("Depth must be a number");
    let slots = args[2].parse::<usize>().expect("Slots must be a number");

    let time = std::time::Instant::now();
    let mut table = QuadraticProbingTable64::new(1 << slots);
    let elapsed = time.elapsed();
    println!("Allocation took: {:?}", elapsed);

    let state = Connect4::initial_state();

    let time = std::time::Instant::now();
    let count1 = count_states::<Connect4, _>(&state, depth, &mut table);
    let elapsed = time.elapsed();

    println!("Perft({}) = {}, or ~2^{:.2}", depth, count1, (count1 as f64).log2());
    println!("Elapsed (Cache): {:?}", elapsed);
    println!("Table size: {}", table.size());
    println!("Load factor: {}", table.load_factor());


    // 1, 7, 49, 238, 1120, 4263, 16422, 54859, 184275, 558186, 1662623, 4568683,
    // 12236101, 30929111, 75437595, 176541259, 394591391, 858218743, 1763883894,
    // 3568259802, 6746155945, 12673345045, 22010823988, 38263228189, 60830813459,
    // x97266114959, 140728569039
}

#[inline]
fn count_states<G: Game, T: TranspositionTable<G::State, ()>>(current_state: &G::State, depth: usize, table: &mut T) -> usize
    where G::State: TranspositionHash
{
    if depth == 0 {
        return 1;
    }

    let mut count = 0;
    for state in current_state.substates() {
        if table.get(&state).is_none() {
            table.insert(state.clone(), ());
            count += count_states::<G, T>(&state, depth - 1, table);
        }
    }
    count
}
