pub mod cube;

use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufWriter, Write},
};

use cube::{
    CUBE_SOLVED_STATE, NUM_MOVES, apply_move, decode_cube_state, decode_pretty_print_moves,
    encode_cube_state, invert_moves,
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

    let mut massive_table: Vec<([u8; 14], Vec<u8>)> = Vec::new();

    for (state, solution) in seen_states.into_iter() {
        massive_table.push((decode_cube_state(state), invert_moves(solution)));
    }

    massive_table.sort_by_key(|(_, sol)| sol.len());

    save_massive_table_to_file(&massive_table);

    println!("done!");
}

fn save_massive_table_to_file(table: &Vec<([u8; 14], Vec<u8>)>) {
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
}
