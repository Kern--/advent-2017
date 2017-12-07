use std::u32;
use std::collections::HashSet;

use util;

/// Finds the index and block count of the memory bank with the largest number of block
fn find_max(memory: &[u32]) -> (usize, u32) {
    let mut i_max = 0usize;
    let mut max = u32::MIN;
    for (i, value) in memory.iter().enumerate() {
        if *value > max {
            i_max = i;
            max = *value;
        }
    }
    (i_max, max)
}

/// Redistributes the memory bank with the highest number of blocks evenly across all banks
fn redistribute(memory: &mut [u32]) {
    let length = memory.len() as u32;
    let (i_max, max) = find_max(memory);
    memory[i_max] = 0;
    for j in 0..length {
        // Calculate the equivalent index of j in the rotate array where
        //  i_max is at length - 1. In this rotated array, the remainder
        //  when the amount to distribute < length will go to the first `remainder`
        //  banks in the array. i.e. the first `remainder` banks will get 1 more block
        let j_rotated = (length - 1 - i_max as u32 + j) % length;
        let remainder = max % length;
        // Each memory bank gets max/length blocks (= the number of times we can give 1 block to each bank without running out)
        // +1 if the rotated index is in the first `remainder` banks. 
        // Justification for 2*remainder / (remainder + j_rotated + 1):
        //  1) This is a u32 division, so it will do a floor by default
        //  2) 2*remainder > remainder + j_rotated for all j_rotated < remainder
        //  3) remainder <= remainder + j_rotated for all j_rotated
        //  4) +1 prevents divide by 0 since j_rotated is 0 indexed (i.e. the 1st entry as at j_rotated = 0)
        // Since remainder <= remainder + j_rotated < 2*remainder, the division will always be either 
        //  1, if the sum < 2*remainder (i.e. j_rotated < remainder)
        //  0, if the sum >= 2*remainder (i.e. j_rotated >= remainder)
        // This is exactly how we want to redistribute the remaining blocks.
        memory[j as usize] += max/length + (2*remainder) / (remainder+ j_rotated + 1);
    }
}

/// Calculates the number of redistrubtion cycles before returning to a previously-encoutered state
pub fn detect_redistribution_loop(memory: &mut [u32]) -> usize {
    let mut cycles = 0;
    let mut previous_states = HashSet::<String>::new();

    while !previous_states.contains(&util::number_slice_to_string(memory)) {
        cycles += 1;
        previous_states.insert(util::number_slice_to_string(memory));
        redistribute(memory);
    }

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_redistribution_loop() {
        let mut memory = vec![0, 2, 7, 0];
        assert_eq!(detect_redistribution_loop(&mut memory), 5);
    }
}