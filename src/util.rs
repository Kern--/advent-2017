/// Converts a str of digits into a vec of numbers.
/// returns None if a digit cannot be converted
///
/// # example
/// ```
/// let input: &'static str = "1234";
/// let output = string_to_number_slice(input).ok();
/// // output = vec![1, 2, 3, 4]
/// ```
pub fn string_to_number_slice(input: &str) -> Option<Vec<u32>> {
    input.chars().map(|c| c.to_digit(10)).collect()
}

pub fn separated_string_to_number_slice(input: &str, separator: &str) -> Option<Vec<u32>> {
    input.split(separator).map(|s| s.parse::<u32>().ok()).collect()
}

pub fn string_to_number_table(input: &str) -> Option<Vec<Vec<u32>>> {
    input.split("\n").map(|row| row.split("\t").map(|s| s.parse::<u32>().ok()).collect()).collect()
}

pub fn number_slice_to_string(input: &[u32]) -> String {
    input.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(",")
}

/// Counts the number of bits in input which are 1
pub fn count_bits(input: u8) -> u16 {
    let mut count = 0;
    for i in 0..8 {
        count += ((input & (1u8 << i)) >> i) as u16
    }
    count
}

/// Converts a &[u8] to the equivalent hex string
pub fn to_hex_string(bytes: &[u8]) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02X}", b))
                               .collect();
  strs.join("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_to_number_slice() {
        let input: &'static str = "123456789";
        assert_eq!(string_to_number_slice(input), Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]))
    }

    #[test]
    fn test_string_to_number_table() {
        let input: &'static str = "5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8";
        let parsed = string_to_number_table(input);
        let expected = vec![vec![5,1,9,5], vec![7,5,3], vec![2,4,6,8]];
        assert_eq!(parsed, Some(expected));
    }
    
    #[test]
    fn test_number_slice_to_string() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(number_slice_to_string(&input), "1,2,3,4,5,6,7,8,9")
    }

    #[test]
    fn test_separated_string_to_number_slice() {
        let input: &'static str = "1\t2\t3\t4\t5";
        assert_eq!(separated_string_to_number_slice(input, "\t"), Some(vec![1,2,3,4,5]));
    }

    #[test]
    fn test_count_bits() {
        let mut input = 1;
        assert_eq!(count_bits(input), 1);

        input = 2;
        assert_eq!(count_bits(input), 1);

        input = 7;
        assert_eq!(count_bits(input), 3);

        input = 0xFF;
        assert_eq!(count_bits(input), 8);
    }
}

