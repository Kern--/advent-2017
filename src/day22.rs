use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
/// An error when parsing a grid
pub enum GridParseError<'a> {
    /// An invalid character encountered when parsing a string to a Grid.
    ///  Only '.' (clean) and '#' (infected) are allowed
    InvalidCharacter(&'a str, usize),
}

impl <'a> Display for GridParseError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            GridParseError::InvalidCharacter(line, column) => write!(f, "Invalid character at column {} in {}", column, line)
        }
    }
}

impl <'a> Error for GridParseError<'a> {
    fn description(&self) -> &str {
        "Invalid Character"
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NodeState {
    Clean,
    Infected,
    Weakend,
    Flagged
}

/// A grid of compute clusters which contains a set of infected nodes
pub struct Grid {
    nodes: HashMap<(i64, i64), NodeState>,
    resistant: bool
}

impl Grid {
    /// Updates the grid at (x, y)
    /// returns (old_state, new_state)
    pub fn update(&mut self, x: i64, y: i64) -> (NodeState, NodeState) {
        let state = self.nodes.entry((x, y)).or_insert(NodeState::Clean);
        // Update state based on simple or resistance of the grid
        let new_state = if self.resistant {
            match *state {
                NodeState::Clean => NodeState::Weakend,
                NodeState::Infected => NodeState::Flagged,
                NodeState::Weakend => NodeState::Infected,
                NodeState::Flagged => NodeState::Clean
            }
        } else {
            match *state {
                NodeState::Clean => NodeState::Infected,
                _ => NodeState::Clean,
            }
        };
        // Update the state and return (old_state, new_state)
        let old_state = (*state).clone();
        *state = new_state.clone();
        (old_state, new_state)
    }
}

impl <'a> TryFrom<&'a str> for Grid {
    type Error = GridParseError<'a>;

    fn try_from(input: &'a str) -> Result<Grid, Self::Error> {
        // Parses a row of '.' and '#' into a list of (x, y) coordinates of infected nodes (i.e. '#')
        //  Takes the y value for this row, and offsets the x value such that the middle of the row is x = 0
        fn parse_row<'a>(row: &'a str, y: i64) -> Result<Vec<(i64, i64)>, GridParseError> {
            // The distance to offset the row such that 0 is centered
            let x_offset = (row.len() as i64) / 2;
            let mut infected_nodes = Vec::new();
            for (column, character) in row.chars().enumerate() {
                match character {
                    '#' => infected_nodes.push((column as i64 - x_offset, y)),
                    '.' => {},
                    _ => return Err(GridParseError::InvalidCharacter(row, column))
                }
            }
            Ok(infected_nodes)
        }

        let mut infected_nodes = HashMap::new();
        let rows = input.split("\n").collect::<Vec<&'a str>>();
        let row_count = rows.len() as i64;
        // The distance to offset the column such that 0 is centered
        let y_offset = row_count / 2;
        let parsed_nodes = rows.iter().enumerate()
            // Parse each row
            //  Offset each row such that 0 is centered, and invert the values so earlier values are positive 
            .map(|(y, row)| parse_row(row, row_count - 1 - y as i64 - y_offset))
            .flat_map(|rows| rows.into_iter())
            .flat_map(|infected_nodes| infected_nodes.into_iter());
        for (x, y) in parsed_nodes {
            infected_nodes.insert((x, y), NodeState::Infected);
        }
        Ok(Grid {nodes: infected_nodes, resistant: false})
    }
}

pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up
        }
    }

    pub fn turn_right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down
        }
    }

    pub fn reverse(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left
        }
    }

    fn step(&self, x: i64, y: i64) -> (i64, i64) {
        match *self {
            Direction::Up => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y - 1),
            Direction::Right => (x + 1, y)
        }
    }
}

/// A virus which tracks a grid of compute nodes which may or may not be infected
pub struct Virus {
    pub grid: Grid,
}

impl Virus {
    pub fn new(mut grid: Grid, resistant: bool) -> Virus {
        grid.resistant = resistant;
        Virus {grid}
    }

    pub fn run(&mut self, bursts: usize) -> u64 {
        let mut infection_count = 0;
        let mut direction = Direction::Up;
        let mut x = 0i64;
        let mut y = 0i64;
        for _ in 0..bursts {
            let (old_state, new_state) = self.grid.update(x, y);
            if new_state == NodeState::Infected {
                infection_count += 1;
            }
            direction = match old_state {
                NodeState::Clean => direction.turn_left(),
                NodeState::Weakend => direction,
                NodeState::Infected => direction.turn_right(),
                NodeState::Flagged => direction.reverse()
            };
            
            let (a, b) = direction.step(x, y);
            x = a;
            y = b;
        }
        infection_count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_grid() {
        let input = "#..\n.#.\n.##";
        let grid = Grid::try_from(input).unwrap();
        assert_eq!(grid.nodes.len(), 4);
        assert_eq!(grid.nodes.get(&(-1, 1)), Some(&NodeState::Infected), "Grid did not contain (-1, 1)");
        assert_eq!(grid.nodes.get(&(0, 0)), Some(&NodeState::Infected), "Grid did not contain (0, 0)");
        assert_eq!(grid.nodes.get(&(0, -1)), Some(&NodeState::Infected), "Grid did not contain (0, -1)");
        assert_eq!(grid.nodes.get(&(1, -1)), Some(&NodeState::Infected), "Grid did not contain (1, -1)");
    }

    #[test]
    fn test_update_grid() {
        let input = "...\n.#.\n...";
        let mut grid = Grid::try_from(input).unwrap();
        assert_eq!(grid.nodes.get(&(0, 0)), Some(&NodeState::Infected), "Grid did not contain (0, 0) before cleaning");
        assert_eq!(grid.update(0, 0), (NodeState::Infected, NodeState::Clean), "Grid did not clean (0, 0) during clean");
        assert_eq!(grid.nodes.get(&(0, 0)), Some(&NodeState::Clean), "Grid contained (0, 0) after cleaning");
        assert_eq!(grid.update(0, 0), (NodeState::Clean, NodeState::Infected), "Grid did not infect (0, 0) during infection");
        assert_eq!(grid.nodes.get(&(0, 0)), Some(&NodeState::Infected), "Grid did not contain (0, 0) after infection");

    }

    #[test]
    fn test_simple_virus() {
        let input = "..#\n#..\n...";
        let grid = Grid::try_from(input).unwrap();
        let mut virus = Virus { grid };
        assert_eq!(virus.run(10000), 5587);
    }

    #[test]
    fn test_resistant_virus() {
        let input = "..#\n#..\n...";
        let mut grid = Grid::try_from(input).unwrap();
        grid.resistant = true;
        let mut virus = Virus { grid };
        assert_eq!(virus.run(10000000), 2511944);
    }
}