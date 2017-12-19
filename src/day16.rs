use std::fmt;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Step {
    Spin(u8),
    Exchange(u8, u8),
    Partner(char, char)
}

impl Step {
    /// Parses a spin step
    /// 
    /// # Args
    /// * `distance` - The distance to spin left
    fn parse_spin(distance: &str) -> Option<Step> {
        if let Some(distance) = distance.parse::<u8>().ok() {
            return Some(Step::Spin(distance));
        }
        None
    }

    /// Parses an exchange step
    /// 
    /// # Args
    /// * `a` - the first index to exchange
    /// * `b` - the second index to exchange
    fn parse_exchange(a: &str, b: &str) -> Option<Step> {
        if let Some(a) = a.parse::<u8>().ok() {
            if let Some(b) = b.parse::<u8>().ok() {
                return Some(Step::Exchange(a, b));
            }
        }
        None
    }

    /// Parses a partner step
    /// 
    /// # Args
    /// * `a` - the first element to partner
    /// * `b` - the second element to partner
    fn parse_partner(a: &str, b: &str) -> Option<Step> {
        if let Some(a) = a.chars().next() {
            if let Some(b) = b.chars().next() {
                if a <= 'p' && b <= 'p' {
                    return Some(Step::Partner(a, b));
                }
            }
        }
        None
    }

    fn parse(input: &str) -> Option<Step> {
        // I'm sure there's a much cleaner way to handle this, but I don't really want to pull
        //  in a proper parsing lib or build one myself
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"(?:(?:s(\d+))|(?:x(\d+)/(\d+))|(?:p(\w)/(\w)))").unwrap();
        }
        let captures = REGEX.captures(input).unwrap();
        match &captures[0][0..1] {
            "s" => Step::parse_spin(&captures[1]),
            "x" => Step::parse_exchange(&captures[2], &captures[3]),
            "p" => Step::parse_partner(&captures[4], &captures[5]),
            _ => None
        }
    }
}

/// Represents the state of of set of dancing programs
/// 
/// A small optimization is made here by noting that a rotation
///  can be represented by tracking the index of the 0th item
///  and then treating the programs as cyclic. This allows us to
///  simply update `offset` on a spin step rather than actually copying memory
pub struct Dance {
    offset: u8,
    programs: Vec<char>
}

impl Dance {
    /// Creates a new dance of 16 programs
    pub fn new() -> Dance {
        Dance::new_sized(16)
    }

    /// Creates a new dance of `size` programs
    pub fn new_sized(size: u8) -> Dance {
        let mut programs = Vec::new();
        for i in 0u8..size {
            programs.push(('a' as u8 + i) as char);
        }
        Dance { offset: 0, programs }
    }

    /// Converts an index into the dance into the real index into
    ///   the vec of programs
    fn get_real_index(&self, index: u8) -> usize {
        ((index + self.offset) % self.programs.len() as u8) as usize
    }

    /// Gets the real index of a program
    /// 
    /// # Args
    /// * `c` - the program whose index is requested
    fn get_index(&self, c: char) -> usize {
        let mut index = 0;
        for i in 0..self.programs.len() {
            if self.programs[i] == c {
                index = i;
            }
        }
        index
    }

    /// Takes one step through the dance
    fn step(&mut self, step: &Step) {
        match *step {
            Step::Spin(i) =>  {
                let len = self.programs.len() as u8;
                self.offset = (self.offset + len - i) % len;
            },
            Step::Exchange(a, b) => {
                let index_a = self.get_real_index(a);
                let index_b = self.get_real_index(b);
                self.programs.swap(index_a, index_b)
            },
            Step::Partner(a, b) =>  {
                let index_a = self.get_index(a);
                let index_b = self.get_index(b);
                self.programs.swap(index_a, index_b)
            }
        }
    }

    /// Parses a dance and performs the steps
    pub fn dance(&mut self, input: &str) {
        self.dance_repeatedly(input, 1);
    }

    /// Parses a dance and performs the steps repeatedly `repetitions` times
    pub fn dance_repeatedly(&mut self, input: &str, repetitions: u32) {
        // Parse all the steps
        let steps = input.split(",")
            .map(Step::parse)
            .collect::<Option<Vec<Step>>>()
            .unwrap_or(Vec::new());

        // Keep track of starting states that have been seen
        let mut starting_states = HashMap::new();
        let mut remaining = 0;

        for i in 0..repetitions {
            let state = format!("{}", &self);
            // If we've seen this starting state before, we have a cycle
            //   and so we can skip performing n cycles as we will end in the 
            //   same state. This is equivalent to saying we only need to perform
            //   reptitions - i % cycle_length
            //   more repetitions to find an answer
            if starting_states.contains_key(&state) {
                let entry = starting_states.entry(state).or_insert(0);
                remaining = (repetitions - i) % (i - *entry);
                break;
            }
            starting_states.insert(state, i);
            // Apply all the steps
            (&steps)
            .into_iter()
            .fold((), |_, step| self.step(step));
        }
        // We found a cylce, compute the remaining iterations to finish
        for _ in 0..remaining {
            (&steps)
            .into_iter()
            .fold((), |_, step| self.step(step));
        }

    }
}

impl fmt::Display for Dance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.programs.len() {
            write!(f, "{}", self.programs[self.get_real_index(i as u8)])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_spin() {
        let input = "s14";
        assert_eq!(Some(Step::Spin(14)), Step::parse(input));
    }

    #[test]
    fn test_parse_exchange() {
        let input = "x2/15";
        assert_eq!(Some(Step::Exchange(2, 15)), Step::parse(input));
    }

    #[test]
    fn test_parse_partner() {
        let mut input = "pa/p";
        assert_eq!(Some(Step::Partner('a', 'p')), Step::parse(input));

        input = "pz/r";
        assert_eq!(None, Step::parse(input));
    }

    #[test]
    fn test_dance() {
        let input = "s1,x3/4,pe/b";
        let mut dance = Dance::new_sized(5);
        dance.dance(input);
        assert_eq!(format!("{}", dance), "baedc");
    }

}