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

fn find_divisible(input: &Vec<u32>) -> (u32, u32) {
    let mut copy = input.to_vec();
    copy.sort();
    for i in 1..copy.len() {
        for j in 0..i {
            if copy[j] > copy[i]/2 {
                break;
            }
            if copy[i] % copy[j] == 0 {
                return (copy[j], copy[i])
            }
        }
    }
    panic!("Could not find answer!")
}

pub fn compute_simple_checksum(input: &Vec<Vec<u32>>) -> u32 {
    let checksum = |row| {
        let (min, max) = minmax(row);
        max - min
    };
    input.iter().map(checksum).sum()
}

pub fn compute_complex_checksum(input: &Vec<Vec<u32>>) -> u32 {
    let checksum = |row| {
        let (bottom, top) = find_divisible(row);
        top / bottom
    };
    input.iter().map(checksum).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_checksum() {
        let table = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];
        assert_eq!(compute_simple_checksum(&table), 18);
    }

    #[test]
    fn test_complex_checksum() {
        let table = vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]];
        assert_eq!(compute_complex_checksum(&table), 9)
    }

    #[test]
    fn test_minmax() {
        let input = vec![5, 1, 9, 5];
        assert_eq!(minmax(&input), (1, 9));

        let input2 = vec![7, 5, 3];
        assert_eq!(minmax(&input2), (3, 7));

        let input3 = vec![2, 4, 6, 8];
        assert_eq!(minmax(&input3), (2, 8));

    }

    #[test]
    fn test_find_divisible() {
        let input = vec![5, 9, 2, 8];
        assert_eq!(find_divisible(&input), (2, 8));

        let input2 = vec![9, 4, 7, 3];
        assert_eq!(find_divisible(&input2), (3, 9));

        let input3 = vec![3, 8, 6, 5];
        assert_eq!(find_divisible(&input3), (3, 6));
    }
}