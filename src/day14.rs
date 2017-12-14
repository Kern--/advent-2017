use day10::Knot;
use util;

///  Counts the number of used squares given a key string
pub fn count_used_squares(key: &str) -> u32 {
    let mut num_squares_used = 0;
    for i in 0..128 {
        let mut knot = Knot::new(255);
        let row_key = format!("{}-{}", key, i);
        let hash = knot.compute_hash(&row_key.into_bytes());
        num_squares_used += hash.iter().map(|b| util::count_bits(*b)).sum::<u16>() as u32;
    }
    num_squares_used
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