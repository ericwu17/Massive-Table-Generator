/// A cube state is represented as an array of 14 integers.
/// the first 7 integers represent the permutation of the 7 pieces,
/// and the last 7 integers represent the permutation.
/// The permutation of a piece is either 0, 1, or 2, and is defined as:
///       the number of clockwise twists necessary to get the white/yellow sticker to face up/down.
///
/// Note that there are 7 pieces to keep track of, not 8, because we assume that only F, R, U
/// moves are applied, and therefore the DBL corner always stays in the same location and orientation.
///
/// Also, note that the orientation of all 7 pieces must add up to zero mod 3, which is why `NUM_ORIENTATIONS` is equal t 3^6
pub const CUBE_SOLVED_STATE: [u8; 14] = [0, 1, 2, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0];
pub const NUM_MOVES: u8 = 9;
pub const SEVEN_FACTORIAL: u32 = 5040;
pub const NUM_ORIENTATIONS: u32 = 3u32.pow(6);
pub const NUM_STATES: u32 = SEVEN_FACTORIAL * NUM_ORIENTATIONS;

pub const MOVES: &[&str] = &["U", "U'", "U2", "F", "F'", "F2", "R", "R'", "R2"];

/// # There are 9 possible moves that can be applied: they are:
/// 0: U
/// 1: U'
/// 2: U2
/// 3: F
/// 4: F'
/// 5: F2
/// 6: R
/// 7: R'
/// 8: R2
pub fn apply_move(s: &mut [u8; 14], move_: u8) {
    debug_assert!(move_ < NUM_MOVES);
    match move_ {
        0 => {
            // U
            (s[0], s[1], s[2], s[3]) = (s[3], s[0], s[1], s[2]);
            (s[7], s[8], s[9], s[10]) = (s[10], s[7], s[8], s[9]);
        }
        1 => {
            // U'
            (s[0], s[1], s[2], s[3]) = (s[1], s[2], s[3], s[0]);
            (s[7], s[8], s[9], s[10]) = (s[8], s[9], s[10], s[7]);
        }
        2 => {
            // U2
            (s[0], s[1], s[2], s[3]) = (s[2], s[3], s[0], s[1]);
            (s[7], s[8], s[9], s[10]) = (s[9], s[10], s[7], s[8]);
        }
        3 => {
            // F
            (s[3], s[2], s[5], s[4]) = (s[4], s[3], s[2], s[5]);
            (s[10], s[9], s[12], s[11]) = (
                (s[11] + 1) % 3,
                (s[10] + 2) % 3,
                (s[9] + 1) % 3,
                (s[12] + 2) % 3,
            );
        }
        4 => {
            // F'
            (s[3], s[2], s[5], s[4]) = (s[2], s[5], s[4], s[3]);
            (s[10], s[9], s[12], s[11]) = (
                (s[9] + 1) % 3,
                (s[12] + 2) % 3,
                (s[11] + 1) % 3,
                (s[10] + 2) % 3,
            );
        }
        5 => {
            // F2
            (s[3], s[2], s[5], s[4]) = (s[5], s[4], s[3], s[2]);
            (s[10], s[9], s[12], s[11]) = (s[12], s[11], s[10], s[9]);
        }
        6 => {
            // R
            (s[2], s[1], s[6], s[5]) = (s[5], s[2], s[1], s[6]);
            (s[9], s[8], s[13], s[12]) = (
                (s[12] + 1) % 3,
                (s[9] + 2) % 3,
                (s[8] + 1) % 3,
                (s[13] + 2) % 3,
            );
        }
        7 => {
            // R'
            (s[2], s[1], s[6], s[5]) = (s[1], s[6], s[5], s[2]);
            (s[9], s[8], s[13], s[12]) = (
                (s[8] + 1) % 3,
                (s[13] + 2) % 3,
                (s[12] + 1) % 3,
                (s[9] + 2) % 3,
            );
        }
        8 => {
            // R2
            (s[2], s[1], s[6], s[5]) = (s[6], s[5], s[2], s[1]);
            (s[9], s[8], s[13], s[12]) = (s[13], s[12], s[9], s[8]);
        }
        _ => {
            unreachable!()
        }
    }
}

pub fn encode_cube_state(cube_state: &[u8; 14]) -> u32 {
    let mut orientation_number = 0;

    for i in 0..6 {
        orientation_number += 3_u32.pow(i) * (cube_state[(7 + i) as usize]) as u32;
    }

    let permutation_number = encode_perm(&cube_state[0..7]);

    permutation_number + orientation_number * SEVEN_FACTORIAL
}

pub fn decode_cube_state(cube_state: u32) -> [u8; 14] {
    let mut orientation_number = cube_state / SEVEN_FACTORIAL;
    let permutation_number = cube_state % SEVEN_FACTORIAL;

    let mut cube_state: [u8; 14] = [0; 14];

    let perm = decode_perm(permutation_number);
    for i in 0..7 {
        cube_state[i] = perm[i];
    }
    let mut orientation_sum = 0;
    for i in 0..6 {
        cube_state[7 + i] = (orientation_number % 3) as u8;
        orientation_sum += orientation_number % 3;
        orientation_number = orientation_number / 3;
    }
    cube_state[13] = ((3 - (orientation_sum % 3)) % 3) as u8;

    cube_state
}

fn decode_perm(k: u32) -> [u8; 7] {
    // https://antoinecomeau.blogspot.com/2014/07/mapping-between-permutations-and.html

    let n: usize = 7;

    let mut permuted: [u8; 7] = [255; 7];
    let mut elems: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];

    let mut m: usize = k as usize;

    for i in 0..n {
        let ind = m % (n - i);
        m = m / (n - i);
        permuted[i] = elems[ind];
        elems[ind] = elems[n - i - 1];
    }

    return permuted;
}

fn encode_perm(perm: &[u8]) -> u32 {
    // https://antoinecomeau.blogspot.com/2014/07/mapping-between-permutations-and.html

    let n: usize = 7;

    let mut pos: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];
    let mut elems: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];

    let mut k: u32 = 0;
    let mut m: u32 = 1;

    for i in 0..(n - 1) {
        k += m * pos[perm[i] as usize] as u32;
        m = m * (n as u32 - i as u32);
        pos[elems[n - i - 1] as usize] = pos[perm[i] as usize];
        elems[pos[perm[i] as usize] as usize] = elems[n - i - 1];
    }
    return k;
}

fn encode_move(move_: &str) -> u8 {
    let index = MOVES.iter().position(|s| *s == move_).unwrap();
    index as u8
}

fn decode_move(move_: u8) -> &'static str {
    debug_assert!(move_ < NUM_MOVES);

    MOVES[move_ as usize]
}

fn encode_moves(moves: Vec<&str>) -> Vec<u8> {
    moves.iter().map(|m| encode_move(m)).collect()
}

fn decode_moves(moves: Vec<u8>) -> Vec<&'static str> {
    moves.iter().map(|m| decode_move(*m)).collect()
}

fn pretty_print_moves(moves: Vec<&str>) -> String {
    moves.join(" ")
}

pub fn decode_pretty_print_moves(moves: Vec<u8>) -> String {
    pretty_print_moves(decode_moves(moves))
}
