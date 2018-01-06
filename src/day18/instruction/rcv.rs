use super::{InstructionType, Instruction, RCV_REGISTER};
use super::super::{Environment};

#[derive(Debug, PartialEq, Eq)]
pub struct Rcv {
    register: String
}

impl Instruction for Rcv {
    fn get_type(&self) -> InstructionType {
        InstructionType::Rcv
    }

    fn execute(&self, environment: &mut Environment) -> Option<i64> {
        let value = environment.get(&self.register);
        if value != 0 {
            let snd = environment.get(&RCV_REGISTER);
            environment.set(&self.register, snd);
            return Some(snd);
        }
        None
    }
}

pub fn parse(input: &str) -> Box<Rcv> {
    return Box::new(Rcv {register: input.into()});
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let reg = "a";
        assert_eq!(parse(reg), Box::new(Rcv {register: "a".into()}));
    }

    #[test]
    fn test_execute() {
        let mut environment = Environment::new();
        environment.set(&RCV_REGISTER, 10);
        let instruction = Rcv {register: "a".into()};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 0);

        environment.set(&"a", 1);
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 10);
    }
}