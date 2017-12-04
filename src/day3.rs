/// Calculates the equivalent distance of n to the nearest odd square.
/// 
///  ## Insights 
///  * numbers arranged in a spiral from 1 to n^2 where n is odd form a square with side lengths (n - 1)
///  * the edges of the square which contains n^2 for some odd n are homologus
///  * each edge is symmetric across it's middle point
///  * the homologous edges and mod do not line up at the corners - % is offset by order steps where order is the 
///    index of the odd square in the square (i.e. 1 = 0, 3 = 1, 5 = 2, 7 = 3, etc)
/// 
/// ## Algorithm
/// 1) let n' = alter n such that each homologous edge starts in a corner (i.e. mod and edge align)
/// 2) let d = distance of n' from the corner on it's edge
/// 3) mirror d across the midpoint of the edge (equivalent to calculating the distance to the nearest corner, even if not on the same edge) 
/// 
/// This description is a bit loose - I'm not entirely sure how to describe this rigorously.
fn calculate_equivalence(n: i32, nearest_odd_square: i32) -> i32 {
    let order = nearest_odd_square / 2 - 1;
    let side_length = nearest_odd_square - 1;
    let offset = (nearest_odd_square / 2) % side_length;

    let cornered_n = n + order %side_length;
    let equivalent_side = cornered_n % side_length;
    (equivalent_side - offset).abs()
}

/// Calculates the minimum number of steps to get from an arbitrary cell of
///  a spiral memory to the center of the memory.
pub fn compute_memory_steps(n: u32) -> i32 {
    if n == 0 || n == 1 {
        return 0
    }

    let root_n = (n as f64).sqrt().ceil() as i32; 
    let nearest_odd_square = if root_n % 2 == 0 { root_n + 1 } else { root_n };

    let cacnonical_n = calculate_equivalence(n as i32, nearest_odd_square);

    // the distance from the center for any l = m^2 where m is odd = m - 1
    // canonical_n = the equivalent distance of n to the nearest odd square from the center of the row
    //   this can be equivalently interpreted as the number of steps closer to the center n is than l
    // therefore, the distance to the center is the distance of l to the center - the number of steps closer n is (i.e. canonical_n)
    nearest_odd_square - 1 - cacnonical_n 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_steps() {
        let mut n = 1;
        assert_eq!(compute_memory_steps(n), 0);

        let mut n = 12;
        assert_eq!(compute_memory_steps(n), 3, "Error with n = 12");

        n = 23;
        assert_eq!(compute_memory_steps(n), 2, "Error with n = 23");

        n = 1024;
        assert_eq!(compute_memory_steps(n), 31, "Error with n = 1024");
    }
}