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

#[derive(Debug)]
/// Partially represents a spinlock where only the size and element after 0 are tracked 
pub struct PseudoSpinLock {
    position: u32,
    step_size: u32,
    value_at_first_index: u32
}

impl PseudoSpinLock {
    pub fn new(step_size: u32) -> PseudoSpinLock {
        PseudoSpinLock { position: 0, step_size, value_at_first_index: 0}
    }

    /// Short circuits the PseudoSpinLock by returning the element after element 0 after `steps` insertions
    pub fn short_circuit(&mut self, steps: u32) -> u32 {
        // The trick is that an element can only be the element after 0 if it is inserted at position 1.
        //  If we assume element 0 is always at position 0, then any insertion point i where 1 < i < length
        //   must have at least one element between it and 0 (namely the element at position 1)
        //  If we insert an element at the end of the list which is equivalent to before 0 since the list is circular
        //   then we can trivially rotate the circular list so that 0 stays in position 0.
        //
        // Thus:
        //  1) The latest element to be inserted at position 0 before reaching the end of the list will be the element after 0
        //  2) The state of the whole spinlock doesn't matter, only the position of insertion which only depends on the length of the list
        for i in 1..steps + 1 {
            let insert_point = ((self.position + self.step_size) % i) + 1;
            self.position = insert_point;
            if insert_point == 1 {
                self.value_at_first_index = i;
            }
        }
        self.value_at_first_index
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
