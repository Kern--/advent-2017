use std::u32;

fn minmax(input: &Vec<u32>) -> (u32, u32) {
    // Finding the min and max in one pass can be implemented
    //  as folding the list into a tuple of (min, max).
    // Each accumulation step is asking "Is the current value < previous min or > previous max?"
    //  and if so, replace min/max with the current value. This results a in a single iteration finding both min and max
    let update =  |(mut min, mut max), &current: &u32| -> (u32, u32) {
        min = if current < min { current } else { min };
        max = if current > max { current } else { max };
        (min, max)
    };
    
    input.iter().fold((u32::MAX, u32::MIN), update)
}

pub fn compute_checksum(input: &Vec<Vec<u32>>) -> u32 {
    // 1) convert each row into a tuple of it's min and max value
    // 2) sum (min - max) for each row
    input.iter().map(minmax).fold(0, |total, (min, max)| { total + max - min })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_checksum() {
        let table = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];
        assert_eq!(compute_checksum(&table), 18);
    }
}