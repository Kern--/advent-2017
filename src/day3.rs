use std::collections::HashMap;

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

/// Finds the first value > max in a spiral memory where each 
///  cells value is the sum of all cells surrounding it which have been filled already
///  starting from the center and stepping outward right -> up -> left -> down
pub fn run_stress_test(max: u32) -> u32 {
    let mut sum = 1;
    let mut stepper = Stepper::new();

    // initialize the memo
    let mut memo = HashMap::<(i32, i32), u32>::new();
    memo.insert((0, 0), 1);

    // Repeat until we find an item with value > the max specified
    while sum <= max {
        sum = 0;
        // Step along the spiral
        let (x, y) = stepper.step();
        // Calculate the sum of all existing cells surroudning the current cell
        for i in -1i32..2 {
            for j in -1i32..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                if let Some(entry) = memo.get(&(x + i, y + j)) {
                    sum += *entry;
                }
            }
        }
        // remember the value we just calculated
        memo.insert((x, y), sum);
    }
    sum
}

#[derive(Debug)]
struct Stepper {
    /// The current x position on the spiral with (0, 0) = center
    x: i32,
    /// The current y position on the spiral with (0, 0) = center
    y: i32,
    /// The number of steps to take before changing direction
    steps: u32,
    /// The number of steps taken in the current direction
    i: u32,
    /// The current direction to step on each call to step
    direction: Direction
}

#[derive(Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down
}

impl Stepper {
    pub fn new() -> Stepper {
        Stepper{x: 0, y: 0, steps: 1, i: 0, direction: Direction::Right }
    }

    /// Takes a step along the spiral noting that the general pattern is:
    ///  right 1
    ///  up 1
    ///  left 2
    ///  down 2
    ///  right 3
    ///  up 3
    ///  left 4
    ///  down 4
    ///  ...
    pub fn step(&mut self) -> (i32, i32) {
        if self.i >= self.steps {
            self.direction = match self.direction {
                Direction::Right => Direction::Up,
                Direction::Up => { self.steps += 1; Direction::Left },
                Direction::Left => Direction::Down,
                Direction::Down => { self.steps += 1; Direction::Right }
            };
            self.i = 0;
        }

        self.x = match self.direction {
            Direction::Right => self.x + 1,
            Direction::Left => self.x - 1,
            Direction::Up | Direction::Down => self.x
        };
        self.y = match self.direction {
            Direction::Up => self.y + 1,
            Direction::Down => self.y - 1,
            Direction::Left | Direction::Right => self.y
        };

        self.i += 1;

        (self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_steps() {
        let mut n = 1;
        assert_eq!(compute_memory_steps(n), 0);

        n = 12;
        assert_eq!(compute_memory_steps(n), 3, "Error with n = 12");

        n = 23;
        assert_eq!(compute_memory_steps(n), 2, "Error with n = 23");

        n = 1024;
        assert_eq!(compute_memory_steps(n), 31, "Error with n = 1024");
    }

    #[test]
    fn test_stress_test() {
        let mut max = 59;
        assert_eq!(run_stress_test(max), 122);

        max = 317;
        assert_eq!(run_stress_test(max), 330);

        max = 750;
        assert_eq!(run_stress_test(max), 806);
    }
}