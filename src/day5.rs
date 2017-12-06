pub fn compute_steps_to_exit_maze<F>(maze: &mut [i32], increment: F) -> u32 
    where F: Fn(i32) -> i32
{
    let mut steps = 0;
    let mut i = 0i32;
    while i >= 0 && i < maze.len() as i32 {
        let prev_i = i;
        steps += 1;
        i += maze[i as usize];
        maze[prev_i as usize] += increment(maze[prev_i as usize]);
    }
    steps
}

/// Computes the number of steps needed to exit a maze.
/// 
/// # Maze Rules
/// 1) start at index i = 0
/// 2) Move to index i + maze[i]
/// 3) Increment maze[prev_i] by 1
/// 4) if i is out of maze bounds, done
pub fn compute_steps_to_exit_simple_maze(maze: &mut [i32]) -> u32 {
    compute_steps_to_exit_maze(maze, |_| 1)
}

/// Computes the number of steps needed to exit a maze.
/// 
/// # Maze Rules
/// 1) start at index i = 0
/// 2) Move to index i + maze[i]
/// 3) Increment maze[prev_i] by 1 if maze[i] < 3, else -1
/// 4) if i is out of maze bounds, done
pub fn compute_steps_to_exit_complex_maze(maze: &mut [i32]) -> u32 {
    compute_steps_to_exit_maze(maze, |maze_value| if maze_value > 2 { -1 } else { 1 })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_maze() {
        let mut maze = vec![0, 3, 0, 1, -3];
        assert_eq!(compute_steps_to_exit_simple_maze(&mut maze), 5);
    }

    #[test]
    fn test_complex_maze() {
        let mut maze = vec![0, 3, 0, 1, -3];
        assert_eq!(compute_steps_to_exit_complex_maze(&mut maze), 10);
    }
}