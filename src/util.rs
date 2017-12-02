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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_to_number_slice() {
        let input: &'static str = "123456789";
        assert_eq!(string_to_number_slice(input), Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]))
    }
}