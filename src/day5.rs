/// Computes the number of steps needed to exit a maze.
/// 
/// # Maze Rules
/// 1) start at index i = 0
/// 2) Move to index i + maze[i]
/// 3) Increment maze[prev_i]
/// 4) if i is out of maze bounds, done
pub fn compute_steps_to_exit_maze(maze: &mut [i32]) -> u32 {
    let mut steps = 0;
    let mut i = 0i32;
    while i >= 0 && i < maze.len() as i32 {
        let prev_i = i;
        steps = steps + 1;
        i += maze[i as usize];
        maze[prev_i as usize] += 1;
    }
    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maze() {
        let mut maze = vec![0, 3, 0, 1, -3];
        assert_eq!(compute_steps_to_exit_maze(&mut maze), 5);
    }
}