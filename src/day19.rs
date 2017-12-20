#[derive(Debug, PartialEq, Eq)]
enum PathElement {
    Vertical,
    Horizontal,
    Hub,
    Letter(char),
    Nothing
}

impl PathElement {
    fn parse(input: char) -> PathElement {
        match input {
            '|' => PathElement::Vertical,
            '-' => PathElement::Horizontal,
            '+' => PathElement::Hub,
            'A' ... 'Z' => PathElement::Letter(input),
            _ => PathElement::Nothing
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn step(&self, i: &mut usize, j: &mut usize)  {
        match self {
            &Direction::Up => *i = *i - 1,
            &Direction::Down => *i = *i + 1,
            &Direction::Right => *j = *j + 1,
            &Direction::Left => *j = *j - 1
        }
    }
}

/// A routing diagram which can be traversed 
pub struct Diagram {
    /// The grid of paths where coordinate (0, 0) = top left
    grid: Vec<Vec<PathElement>>
}

impl Diagram {
    // Parses a text input of a routing diagram
    pub fn parse(input: &str) -> Diagram {
        let grid = input.split("\n").map(
            |line| line.chars().map(PathElement::parse).collect::<Vec<PathElement>>()
        ).collect::<Vec<Vec<PathElement>>>();
        Diagram {grid}
    }

    /// Finds the starting coordinates of the routing diagram 
    /// where the starting point is the signular vertical element in the 0th row
    fn find_start(&self) -> (usize, usize) {
        for (i, element) in self.grid[0].iter().enumerate() {
            if element == &PathElement::Vertical {
                return (0, i);
            }
        }
        (0, 0)
    }

    /// Gets the element at the specified index
    /// returns 0 if the index is out of range
    fn get_element<'a>(&'a self, i: usize, j: usize) -> &'a PathElement {
        if i < self.grid.len() && j < self.grid[i].len() {
            return &self.grid[i][j];
        }
        &PathElement::Nothing
    }

    /// Updates the travel direction.
    /// 
    /// The travel direction can only change on hubs (+)
    fn update_direction(&self, i: usize, j: usize, direction: Direction) -> Direction {
        let element = self.get_element(i, j);
        if element != &PathElement::Hub {
            return direction;
        }
        match direction {
            Direction::Up | Direction::Down => {
                if self.get_element(i, j+1) != &PathElement::Nothing {
                    return Direction::Right;
                }
                Direction::Left
            },
            Direction::Left | Direction::Right => {
                if self.get_element(i-1, j) != &PathElement::Nothing {
                    return Direction::Up;
                }
                Direction::Down
            }
        }
    }

    /// Navigates through the diagram, returning the nodes visited in order
    pub fn navigate(&self) -> String {
        let (mut i, mut j) = self.find_start();
        let mut direction = Direction::Down;
        let mut result = String::new();

        // if we hit nothing, then it's impossible to continue in the current direction meaning we're done
        while self.get_element(i, j) != &PathElement::Nothing {
            // update the direction if necessary
            direction = self.update_direction(i, j, direction);
            // step in the current direction
            direction.step(&mut i, &mut j);
            // if we're on a letter, record it.
            if let PathElement::Letter(c) = *self.get_element(i, j) {
                result.push(c);
            }
        }
        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_navigate() {
        let input = r##"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
"##;
        let diagram = Diagram::parse(input);
        assert_eq!(diagram.navigate(), "ABCDEF");
    }
}