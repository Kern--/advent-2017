use super::{InstructionType, Instruction};
use super::super::environment::{Environment, SpecialRegister};
use super::super::value::Value;

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
        environment.set(&SpecialRegister::SND.get_name(), value);
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

    #[test]
    fn test_execute() {
        let mut environment = Environment::new();
        let instruction = Snd {sound:  Value::Literal(8)};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&SpecialRegister::SND.get_name()),  8);
    }

    #[test]
    fn test_execute_register() {
        let mut environment = Environment::new();
        environment.set(&"a", 2);
        let instruction = Snd {sound: Value::Register("a".into())};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 2);
        assert_eq!(environment.get(&SpecialRegister::SND.get_name()),  2);
    }
}