// Model this problem as a 2D grid where only positions abs(x + y) % 2 = 0.
// This forms a grid like the following:
//
// x | o | x | o
// o | x | o | x
// x | o | x | o
// o | x | o | x
//
// where o is a valid position and x is an invalid position.
//
// With such a grid we can define NW, NE, SW, SE as a diagonal step in this grid, i.e. x +/- 1, y +/- 1
//  and N, S as a vertical step. Since the position directly above and below a valid position is not valid,
//      N, S correspond to y +/- 2
//
// To calulate the distance from (0, 0), we first figure out where we are relative to (0, 0)
// If NW, step SE
//    NE, step SW
//    SW, step NE
//    SE, step NW
//    S, step N in straight line
//    N, step S in straight line
// To simplify, we know that if we are on the diagonal, there is a straight line to (0, 0) of length abs(x) or abs(y)
// If we are directly south or north, we are abs(y) / 2 steps away in a straight line since only every other vertical position is valid

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    N,
    Nw,
    Ne,
    S,
    Sw,
    Se
}

/// Parses a string into a vector of directions
fn parse_directions(path: &str) -> Option<Vec<Direction>> {
    let mut directions = Vec::new();
    for direction in path.split(",") {
        match direction {
            "n" => directions.push(Direction::N),
            "nw" => directions.push(Direction::Nw),
            "ne" => directions.push(Direction::Ne),
            "s" => directions.push(Direction::S),
            "sw" => directions.push(Direction::Sw),
            "se" => directions.push(Direction::Se),
            _ => return None
        }
    }
    Some(directions)
}

/// Calculates the final position after following a path containing a series of directions
/// returns (final_x, final_y), (max_x, max_y)
fn calculate_final_position(path: &str) -> ((i64, i64), (i64, i64)) {
    let mut x = 0;
    let mut y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    if let Some(directions) = parse_directions(path) {
        for direction in directions {
            step(&mut x, &mut y, direction);
            if x.abs() + y.abs() > max_x + max_y {
                max_x = x.abs();
                max_y = y.abs();
            }
        }
    }
    ((x, y), (max_x, max_y))
}

/// Calculates the new X and Y position after taking a step in the dir Direction
fn step(x: &mut i64, y: &mut i64, dir: Direction) {
    let (new_x, new_y) = match dir {
        Direction::N => (*x, *y + 2),
        Direction::Nw => (*x - 1, *y + 1),
        Direction::Ne => (*x + 1, *y + 1),
        Direction::S => (*x, *y - 2),
        Direction::Sw => (*x - 1, *y - 1),
        Direction::Se => (*x + 1, *y - 1),
    };
    *x = new_x;
    *y = new_y;
}

/// Calculates the number of steps to get from the specified position to (0, 0)
fn calculate_steps(start_x: i64, start_y: i64) -> u64 {
    let mut steps = 0;
    let mut x = start_x;
    let mut y = start_y;

    loop {
        // Diagonal from (0, 0), each step will take both x and y 1 step closer.
        //  therefore the remaining distance is abs(x) which equals abs(y)
        if x.abs() == y.abs() {
            return steps + x.abs() as u64;
        }
        // Are we east?
        if x > 0 {
            // Are we north?
            if y > 0 {
                // We are NE, go SW
                step(&mut x, &mut y, Direction::Sw);
            } else {
                // We are SE, go NW
                step(&mut x, &mut y, Direction::Nw);
            }
            steps += 1;
        // Are we west?
        } else if x < 0 {
            // are we north?
            if y > 0 {
                // We are NW, go SE
                step(&mut x, &mut y, Direction::Se);
            } else {
                // We are SW, go NE
                step(&mut x, &mut y, Direction::Ne);
            }
            steps += 1;
        // We must be north or south. which means we're exactly abs(y/2) steps away
        } else {
            return steps + (y.abs() / 2) as u64;
        }
    }
}

/// Computes the shortest distance from the final position after following path to (0, 0)
/// returns (current_distance, max_distance)
pub fn compute_distance(path: &str) -> (u64, u64) {
    let ((final_x, final_y), (max_x, max_y)) = calculate_final_position(path);
    println!("({}, {})", final_x, final_y);
    let current_distance = calculate_steps(final_x, final_y);
    let max_distance = calculate_steps(max_x, max_y);
    (current_distance, max_distance)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_directions() {
        let input = "n,nw,ne,s,sw,se";
        let expected = vec![Direction::N, Direction::Nw, Direction::Ne, Direction::S, Direction::Sw, Direction::Se];
        assert_eq!(parse_directions(input).unwrap(), expected);
    }

    #[test]
    fn test_distances() {
        let mut input = "ne,ne,ne";
        let (distance, _) = compute_distance(input);
        assert_eq!(distance, 3); 

        input = "ne,ne,sw,sw";
        let (distance, _) = compute_distance(input);
        assert_eq!(distance, 0); 

        input = "ne,ne,s,s";
        let (distance, _) = compute_distance(input);
        assert_eq!(distance, 2); 

        input = "se,sw,se,sw,sw";
        let (distance, _) = compute_distance(input);
        assert_eq!(distance, 3); 
    }

}