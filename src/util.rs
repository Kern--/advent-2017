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

pub fn string_to_number_table(input: &str) -> Option<Vec<Vec<u32>>> {
    input.split("\n").map(|row| row.split("\t").map(|s| s.parse::<u32>().ok()).collect()).collect()
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
}

