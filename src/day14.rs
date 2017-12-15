use std::collections::HashSet;

use day10::Knot;
use util;

type FragmentState = Vec<[u8;16]>;

fn compute_fragment_state(key: &str) -> FragmentState {
    let mut state = FragmentState::new();
    for i in 0..128 {
        let mut knot = Knot::new(255);
        let row_key = format!("{}-{}", key, i);
        let hash = knot.compute_hash(&row_key.into_bytes());
        state.push(hash);
    }
    state
}

///  Counts the number of used squares given a key string
pub fn count_used_squares(key: &str) -> u32 {
    compute_fragment_state(key)
        .into_iter()
        .map(|row|
            row.iter()
            .map(|b| util::count_bits(*b) as u32)
            .sum::<u32>())
        .sum::<u32>()
}

/// Converts a bit index from 0 to 128*128 to a byte-coordinate plus offset
/// (i, j) index to a specific byte in a 2D array of bytes and offset indexes to a specific bit into that byte
fn bit_num_to_coordinate(bit: u32) -> (u32, u32, u32) {
    let i = bit / 128;
    let j = (bit % 128) / 8;
    let offset = (bit % 128) % 8;
    (i, j, offset)
}

/// Checks whether a bit indexed from 0 to 128*128 is set to 1 in a Fragment State
fn is_bit_set(state: &FragmentState, bit: u32) -> bool {
    let (i, j, offset) = bit_num_to_coordinate(bit);
    is_coordinate_set(state, i, j, offset)
}

/// Checks whether a byte (x, y) coordiante + a bit offset is set to 1 within a FragmentState
fn is_coordinate_set(state: &FragmentState, i: u32, j: u32, offset: u32) -> bool {
    // 7 - offset because coordinates are indexed from left to right, but bytes are index from right to left
    state[i as usize][j as usize] & (1 << (7 - offset)) != 0
}

/// Converts a bit indexed from 0 to 128*128 into an (x, y) coordinate
///  of (0 to 128, 0 to 128)
fn bit_to_bit_coordinate(bit: u32) -> (u32, u32) {
    (bit / 128, bit % 128)
}

/// Converts an (x, y) coordinate of (0 to 128, 0 to 128)
///  into a bit indexed from 0 to 128*128 into
fn bit_coordinate_to_bit(x: u32, y: u32) -> u32 {
    x * 128 + y
}

/// Updates the that contains `bit` recursively
fn get_group(state: &FragmentState, group: &mut HashSet<u32>, bit: u32) {
    if group.contains(&bit) || !is_bit_set(state, bit) {
        return;
    }
    group.insert(bit);
    let (x, y) = bit_to_bit_coordinate(bit);
    if x != 0 {
        get_group(state, group, bit_coordinate_to_bit(x - 1, y));
    }
    if x != 127 {
        get_group(state, group, bit_coordinate_to_bit(x + 1, y));
    }
    if y != 0 {
        get_group(state, group, bit_coordinate_to_bit(x, y - 1));
    }
    if y != 127 {
        get_group(state, group, bit_coordinate_to_bit(x, y + 1));
    }
}

/// Gets all groups of adjacent squares that are in use
/// 
/// Each square is assigned a number from 0 to 128*128 where the squares are indexed from
/// 0, 1, 2, ... 127
/// 128, 129, 130 ... 255
/// etc
pub fn get_groups(key: &str) -> Vec<HashSet<u32>> {
    let state = compute_fragment_state(key);
    let mut groups = Vec::<HashSet<u32>>::new();
    for bit in 0..128*128 {
        if !(&groups).into_iter().any(|g| g.contains(&bit)) && is_bit_set(&state, bit) {
            let mut group = HashSet::new();
            get_group(&state, &mut group, bit);
            groups.push(group);
        }
    }
    groups
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cound_used_squares() {
        let key = "flqrgnkx";
        assert_eq!(count_used_squares(key), 8108);
    }
}