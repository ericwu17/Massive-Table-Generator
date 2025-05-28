pub mod cube;

use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufWriter, Write},
};

use cube::{
    CUBE_SOLVED_STATE, NUM_MOVES, NUM_STATES, apply_move, decode_cube_state,
    decode_pretty_print_moves, encode_cube_state, invert_moves,
};

fn main() {
    let solved_state = encode_cube_state(&CUBE_SOLVED_STATE);

    let mut states_to_visit: VecDeque<u32> = VecDeque::new();

    states_to_visit.push_back(solved_state);

    let mut seen_states: HashMap<u32, Vec<u8>> = HashMap::new();
    seen_states.insert(solved_state, Vec::new());

    while !states_to_visit.is_empty() {
        let s = states_to_visit.pop_front().unwrap();
        let moves = seen_states.get(&s).unwrap().clone();

        for m in 0..NUM_MOVES {
            let state = decode_cube_state(s);
            let mut new_state = state.clone();
            apply_move(&mut new_state, m);
            let new_state = encode_cube_state(&new_state);
            if !seen_states.contains_key(&new_state) {
                states_to_visit.push_back(new_state);

                let mut opt_solution = moves.clone();
                opt_solution.push(m);
                seen_states.insert(new_state, opt_solution);
            }
        }
    }

    println!(
        "Finished exploring all the states. Total number of states is {}",
        seen_states.len()
    );

    let mut massive_table: Vec<([u8; 14], Vec<u8>)> = Vec::new();

    for (state, solution) in seen_states.into_iter() {
        massive_table.push((decode_cube_state(state), invert_moves(solution)));
    }

    println!("Inserted all inverses into a massive table");

    save_massive_table_to_files(massive_table);

    println!("Done!");
}

fn save_massive_table_to_files(mut table: Vec<([u8; 14], Vec<u8>)>) {
    table.sort_by_key(|(_, sol)| sol.len());

    let file = File::create("massive_table.txt").unwrap();
    let mut file_buf_writer = BufWriter::new(file);

    for (state, sol) in table.clone().into_iter() {
        let sol = decode_pretty_print_moves(sol);

        let state = serde_json::to_string(&state).unwrap();

        file_buf_writer.write(state.as_bytes()).unwrap();
        file_buf_writer.write(" ".as_bytes()).unwrap();
        file_buf_writer.write(sol.as_bytes()).unwrap();
        file_buf_writer.write("\n".as_bytes()).unwrap();
    }

    let mut solutions: Vec<Option<Vec<u8>>> = vec![None; NUM_STATES as usize];
    assert_eq!(table.len(), solutions.len());
    for (state, sol) in table.clone().into_iter() {
        let state_idx = encode_cube_state(&state) as usize;
        let sol_bytes = encode_solution_as_bytes(sol);
        assert!(solutions[state_idx].is_none());
        solutions[state_idx] = Some(sol_bytes);
    }

    let binary_file = File::create("massive_table.bin").unwrap();
    let mut bin_file_buf_writer = BufWriter::new(binary_file);

    for idx in 0..solutions.len() {
        let sol = solutions[idx].as_deref().unwrap();
        assert!(sol.len() == 6);
        bin_file_buf_writer.write(sol).unwrap();
    }
}

fn encode_solution_as_bytes(sol: Vec<u8>) -> Vec<u8> {
    // encode the solution as a 6 byte chunk for the binary lookup table
    assert!(sol.len() <= 12);

    let mut sol_padded: Vec<u8> = sol;

    while sol_padded.len() < 12 {
        sol_padded.push(15); // use 15 = 0b1111 as a marker for an empty move.
    }
    assert!(sol_padded.len() == 12);

    let mut result = Vec::new();

    for i in 0..6 {
        let first = sol_padded[2 * i];
        let second = sol_padded[2 * i + 1];

        let combined = first + (second << 4);

        result.push(combined)
    }

    result
}
