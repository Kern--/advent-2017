use super::{InstructionType, Instruction};
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
        let received = environment.receive();
        match received {
            Some(value) => {
                environment.set(&self.register, value);
                Some(value)
            },
            None => {
                environment.jump_pc(0);
                None
            }
        }       
    }
}

pub fn parse(input: &str) -> Box<Rcv> {
    return Box::new(Rcv {register: input.into()});
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_parse() {
        let reg = "a";
        assert_eq!(parse(reg), Box::new(Rcv {register: "a".into()}));
    }
}