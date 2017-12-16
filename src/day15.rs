pub struct Generator {
    factor: u64,
    state: u64
}


impl Generator {
    pub fn new(factor: u64, initial_value: u64) -> Generator {
        Generator { factor, state: initial_value}
    }

    /// Generates numbers according to the formula:
    /// n[i] = n[i-1] * factor % 2147483647
    pub fn generate(&mut self) -> u64 {
        self.state = (self.state * self.factor) % 2147483647;
        self.state
    }
}

pub struct Judge {
    a: Generator,
    b: Generator,
    mask: u64
}

impl Judge {
    pub fn new(a: Generator, b: Generator) -> Judge {
        Judge { a, b, mask: 0xFFFF }
    }

    /// Counts the number of generated numbers from 2 generators (a, b)
    ///  for which the a.generate() & mask = b.generate() & mask
    ///  after 40,000,000 trials
    pub fn judge(&mut self) -> u32 {
        self.judge_trials(40_000_000)
    }

    /// Counts the number of generated numbers from 2 generators (a, b)
    ///  for which the a.generate() & mask = b.generate() & mask
    ///  after a specified number of trials
    pub fn judge_trials(&mut self, trials: u32) -> u32 {
        (0..trials).filter(|_| self.a.generate() & self.mask == self.b.generate() & self.mask).map(|_| 1).sum::<u32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generator() {
        let mut generator = Generator::new(16807, 65);
        assert_eq!(generator.generate(), 1092455);
        assert_eq!(generator.generate(), 1181022009);
        assert_eq!(generator.generate(), 245556042);
        assert_eq!(generator.generate(), 1744312007);
        assert_eq!(generator.generate(), 1352636452);
    }

    #[test]
    fn test_judge() {
        let a = Generator::new(16807, 65);
        let b = Generator::new(48271, 8921);
        let mut judge = Judge::new(a, b);
        assert_eq!(judge.judge(), 588);
    }
}