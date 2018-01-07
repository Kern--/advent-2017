use super::{InstructionType, Instruction};
use super::super::{Environment, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct Snd {
    sound: Value
}

impl Instruction for Snd {
    fn get_type(&self) -> InstructionType {
        InstructionType::Snd
    }

    fn execute(&self, environment: &mut Environment) -> Option<i64> {
        let value = environment.get_value(&self.sound);
        environment.send(value);
        Some(value)
    }
}

pub fn parse(input: &str) -> Box<Snd> {
    let sound = Value::parse(input);
    return Box::new(Snd {sound});
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let reg = "12";
        assert_eq!(parse(reg), Box::new(Snd {sound: Value::Literal(12)}));
    }
}