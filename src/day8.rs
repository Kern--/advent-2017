use std::collections::HashMap;
use regex::Regex;
use std::i32;

/// Environment maps registers to values
type Environment = HashMap<String, i32>;

/// A condition that can be checked against the environment
trait Condition<'a> {
    fn is_satisfied(&self, &Environment) -> bool;
}

/// A mutation that can be made on the environment
trait Operation<'a> {
    // performs an operation on the environment and returns the modified register
    fn perform(&self, &mut Environment) -> (&'a str, i32);
}

/// A representation of an increment of a register in the environment
/// Increments `register` by `value` if `condition` holds in the current environment
struct Inc<'a> {
    register: &'a str,
    value: i32,
    condition: Box<Condition<'a> + 'a>
}

impl <'a> Operation<'a> for Inc<'a> {
    fn perform(&self, environment: &mut Environment) -> (&'a str, i32) {
        if self.condition.is_satisfied(environment) {
            let current_value = *environment.get(self.register).unwrap_or(&0);
            let new_value = current_value + self.value;
            *environment.entry(String::from(self.register)).or_insert(0) = new_value;
        }
        let entry = environment.entry(String::from(self.register)).or_insert(0);
        (self.register, *entry)
    }
}

/// A representation of an decrement of a register in the environment
/// Decrements `register` by `value` if `condition` holds in the current environment
struct Dec<'a> {
    register: &'a str,
    value: i32,
    condition: Box<Condition<'a> + 'a>
}

impl <'a> Operation<'a> for Dec<'a> {
    fn perform(&self, environment: &mut Environment) -> (&'a str, i32) {
        if self.condition.is_satisfied(environment) {
            let current_value = *environment.get(self.register).unwrap_or(&0);
            let new_value = current_value - self.value;
            *environment.entry(String::from(self.register)).or_insert(0) = new_value;
        }
        let entry = environment.entry(String::from(self.register)).or_insert(0);
        (self.register, *entry)
    }
}

/// A representation of an equals condition
/// returns true if `register` == `value` in the current environment
struct Eq<'a> {
    register: &'a str,
    value: i32,
}

impl <'a> Condition<'a> for Eq<'a> {
    fn is_satisfied(&self, environment: &Environment) -> bool {
        let register = environment.get(&String::from(self.register)).unwrap_or(&0);
        *register == self.value
    } 
}

/// A representation of an less than condition
/// returns true if `register` < `value` in the current environment
struct Lt<'a> {
    register: &'a str,
    value: i32,
}


impl <'a> Condition<'a> for Lt<'a> {
    fn is_satisfied(&self, environment: &Environment) -> bool {
        let register = environment.get(&String::from(self.register)).unwrap_or(&0);
        *register < self.value
    } 
}

/// A representation of an greater than condition
/// returns true if `register` > `value` in the current environment
struct Gt<'a> {
    register: &'a str,
    value: i32,
}

impl <'a> Condition<'a> for Gt<'a> {
    fn is_satisfied(&self, environment: &Environment) -> bool {
        let register = environment.get(&String::from(self.register)).unwrap_or(&0);
        *register > self.value
    } 
}

/// A representation of an less than or equal condition
/// returns true if `register` <= `value` in the current environment
struct Lte<'a> {
    register: &'a str,
    value: i32,
}

impl <'a> Condition<'a> for Lte<'a> {
    fn is_satisfied(&self, environment: &Environment) -> bool {
        let register = environment.get(&String::from(self.register)).unwrap_or(&0);
        *register <= self.value
    } 
}

/// A representation of an greater than or equal condition
/// returns true if `register` >= `value` in the current environment
struct Gte<'a> {
    register: &'a str,
    value: i32,
}

impl <'a> Condition<'a> for Gte<'a> {
    fn is_satisfied(&self, environment: &Environment) -> bool {
        let register = environment.get(&String::from(self.register)).unwrap_or(&0);
        *register >= self.value
    } 
}

/// A representation of an not equal condition
/// returns true if `register` != `value` in the current environment
struct Ne<'a> {
    register: &'a str,
    value: i32,
}

impl <'a> Condition<'a> for Ne<'a> {
    fn is_satisfied(&self, environment: &Environment) -> bool {
        let register = environment.get(&String::from(self.register)).unwrap_or(&0);
        *register != self.value
    } 
}

/// A struct which can interpret operations and apply their results to an environment 
pub struct Interpreter<'a> {
    operations: Vec<Box<Operation<'a> + 'a>>,
    environment: HashMap<String, i32>,
    // The largest value seen by the interpreter at any point during execution
    largest_value: i32,
}

impl <'a> Interpreter<'a> {
    /// Parses an input into a series of sequential operations 
    ///  that are stored inside the Interpreter
    pub fn from_str(input: &'a str) -> Option<Interpreter<'a>> {
        let lines = input.split("\n");
        let regex = Regex::new(r"(\pL+) (\pL+) (.+) if (\pL+) (.+) (.+)").unwrap();
        fn parse<'a>(line: &'a str, regex: &Regex) -> Option<Box<Operation<'a> + 'a>> {
            if let Some(captures) = regex.captures(line) {
                let register: &'a str = captures.get(1).map_or("", |m| m.as_str());
                let op: &'a str = captures.get(2).map_or("", |m| m.as_str());
                let value = captures.get(3).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
                let cond_register: &'a str = captures.get(4).map_or("", |m| m.as_str());
                let cond: &'a str = captures.get(5).map_or("", |m| m.as_str());
                let cond_value = captures.get(6).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
                let condition: Box<Condition<'a> + 'a> = match cond {
                    "==" => Box::new(Eq {register: cond_register, value: cond_value}),
                    ">" => Box::new(Gt {register: cond_register, value: cond_value}),
                    "<" => Box::new(Lt {register: cond_register, value: cond_value}),
                    ">=" => Box::new(Gte {register: cond_register, value: cond_value}),
                    "<=" => Box::new(Lte {register: cond_register, value: cond_value}),
                    "!=" => Box::new(Ne {register: cond_register, value: cond_value}),
                    _ => panic!("Unknown Condition {}", cond)
                };
                let operation: Box<Operation<'a> + 'a> = match op {
                    "inc" => Box::new(Inc { register, value, condition}),
                    "dec" => Box::new(Dec { register, value, condition}),
                    _ => panic!("Unkown Operation {}", op)
                };
                return Some(operation)
            }
            println!("Couldn't parse: {}", line);
            None
        }
        if let Some(operations) = lines.map(|s| parse(s, &regex)).collect::<Option<Vec<Box<Operation<'a>>>>>() {
            return Some(Interpreter {operations, environment: HashMap::new(), largest_value: i32::MIN })
        }
        None
    }

    /// Executes the series of instructions held inside the interpreter
    pub fn execute(&mut self) {
        for op in &self.operations {
            let (_, value) = op.perform(&mut self.environment);
            if value > self.largest_value {
                self.largest_value = value;
            }
        }
    }

    /// Gets the largest value in the current environment
    pub fn get_current_largest_value(&self) -> i32 {
        let mut largest = i32::MIN;
        for (_, value) in &self.environment {
            if *value > largest {
                largest = *value;
            }
        }
        largest
    }

    /// Gets the largest value seen at any point during execution
    pub fn get_largest_value(&self) -> i32 {
        self.largest_value
    }

    #[allow(dead_code)]
    fn get_value(&mut self, register: &str) -> i32 {
        *self.environment.entry(String::from(register)).or_insert(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_environment_conditions() {
        let environment = HashMap::new();

        // EQ
        let eq = Eq { register: "a", value: 1};
        assert!(!eq.is_satisfied(&environment), "a == 1 when env = {} should have failed");
        let eq = Eq { register: "a", value: 0};
        assert!(eq.is_satisfied(&environment), "a == 0 when env = {} should have passed");

        // LT
        let lt = Lt { register: "a", value: -1};
        assert!(!lt.is_satisfied(&environment), "a < -1 when env = {} should have failed");
        let lt = Lt { register: "a", value: 1};
        assert!(lt.is_satisfied(&environment), "a < 1 when env = {} should have passed");

        // GT
        let gt = Gt { register: "a", value: 1};
        assert!(!gt.is_satisfied(&environment), "a > 1 when env = {} should have failed");
        let gt = Gt { register: "a", value: -1};
        assert!(gt.is_satisfied(&environment), "a > -1 when env = {} should have passed");

        // NE
        let ne = Ne { register: "a", value: 0};
        assert!(!ne.is_satisfied(&environment), "a != 0 when env = {} should have failed");
        let ne = Ne { register: "a", value: 1};
        assert!(ne.is_satisfied(&environment), "a != 1 when env = {} should have passed");

        // LTE
        let lte = Lte { register: "a", value: -1};
        assert!(!lte.is_satisfied(&environment), "a <= -1 when env = {} should have failed");
        let lte = Lte { register: "a", value: 0};
        assert!(lte.is_satisfied(&environment), "a <= 0 when env = {} should have passed");

        // GTE
        let gte = Gte { register: "a", value: 1};
        assert!(!gte.is_satisfied(&environment), "a >= 1 when env = {} should have failed");
        let gte = Gte { register: "a", value: 0};
        assert!(gte.is_satisfied(&environment), "a >= 0 when env = {} should have passed");
    }

    #[test]
    fn test_nonempty_environtment_conditions() {
        let mut environment = HashMap::new();
        environment.insert(String::from("a"), 5);

        // EQ
        let eq = Eq { register: "a",  value: 5 };
        assert!(eq.is_satisfied(&environment), "a == 5 when env = {a: 5} should have passed");

        // LT
        let lt = Lt { register: "a",  value: 6 };
        assert!(lt.is_satisfied(&environment), "a < 6 when env = {a: 5} should have passed");

        // Gt
        let gt = Gt { register: "a",  value: 4 };
        assert!(gt.is_satisfied(&environment), "a > 4 when env = {a: 5} should have passed");

        // LTE
        let lte = Lte { register: "a",  value: 5 };
        assert!(lte.is_satisfied(&environment), "a <= 5 when env = {a: 5} should have passed");

        // GTE
        let gte = Gte { register: "a",  value: 5 };
        assert!(gte.is_satisfied(&environment), "a >= 5 when env = {a: 5} should have passed");

        // NE
        let ne = Ne { register: "a",  value: 4 };
        assert!(ne.is_satisfied(&environment), "a != 4 when env = {a: 5} should have passed");
    }

    #[test]
    fn test_inc_not_satisfied() {
        let mut environment = HashMap::new();
        environment.insert(String::from("a"), 5);

        let cond = Box::new(Eq { register: "a", value: 4 });
        let op = Box::new(Inc { register: "a", value: 5, condition: cond });
        let mut interpreter = Interpreter { environment, operations: vec![op], largest_value: i32::MIN };
        interpreter.execute();
        assert_eq!(interpreter.get_value("a"), 5);
    }

    #[test]
    fn test_inc_satisfied() {
        let mut environment = HashMap::new();
        environment.insert(String::from("a"), 5);

        let cond = Box::new(Eq { register: "a", value: 5 });
        let op = Box::new(Inc { register: "a", value: 5, condition: cond });
        let mut interpreter = Interpreter { environment, operations: vec![op], largest_value: i32::MIN };
        interpreter.execute();
        assert_eq!(interpreter.get_value("a"), 10);
    }

    #[test]
    fn test_dec_not_satisfied() {
        let mut environment = HashMap::new();
        environment.insert(String::from("a"), 5);

        let cond = Box::new(Eq { register: "a", value: 4 });
        let op = Box::new(Dec { register: "a", value: 5, condition: cond });
        let mut interpreter = Interpreter { environment, operations: vec![op], largest_value: i32::MIN };
        interpreter.execute();
        assert_eq!(interpreter.get_value("a"), 5);
    }

    #[test]
    fn test_dec_satisfied() {
        let mut environment = HashMap::new();
        environment.insert(String::from("a"), 5);

        let cond = Box::new(Eq { register: "a", value: 5 });
        let op = Box::new(Dec { register: "a", value: 5, condition: cond });
        let mut interpreter = Interpreter { environment, operations: vec![op], largest_value: i32::MIN };
        interpreter.execute();
        assert_eq!(interpreter.get_value("a"), 0);
    }

    #[test]
    fn test_interpreter() {
        let input = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10";
        let mut interpreter = Interpreter::from_str(input).unwrap();
        interpreter.execute();
        assert_eq!(interpreter.largest_value, 10);
    }
}