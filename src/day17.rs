#[derive(Debug)]
pub struct SpinLock {
    position: u32,
    step_size: u32,
    state: Vec<u32>
}


impl SpinLock {
    /// Creates a new spinlock that steps `step_size` steps after each insertion
    pub fn new(step_size: u32) -> SpinLock {
        let mut state = Vec::new();
        state.push(0);
        SpinLock {position: 0, step_size, state}
    }

    /// Short circuits the spinlock by returning the number that appears after the current position
    ///  after `steps` insertions
    pub fn short_circuit(&mut self, steps: u32) -> u32 {
        let mut insert_point = 0;
        for i in 1..steps + 1 {
            insert_point = ((self.position + self.step_size) % i) + 1;
            self.state.insert(insert_point as usize, i);
            self.position = insert_point;
        }
        let point = (insert_point + 1) as usize % self.state.len();
        self.state[point]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_short_circuit() {
        let mut lock = SpinLock::new(3);
        assert_eq!(lock.short_circuit(2017), 638);
    }
}
