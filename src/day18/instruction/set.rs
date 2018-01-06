use super::{InstructionType, Instruction};
use super::super::Environment;
use super::super::Value;

#[derive(Debug, PartialEq, Eq)]
pub struct Set {
    register: String,
    value: Value
}

impl Instruction for Set {
    fn get_type(&self) -> InstructionType {
        InstructionType::Set
    }

    fn execute(&self, environment: &mut Environment) -> Option<i64> {
        let value = environment.get_value(&self.value);
        environment.set(&self.register, value);
        Some(value)
    }
}

pub fn parse(reg: &str, val: &str) -> Box<Set> {
    let value = Value::parse(val);
    return Box::new(Set {register: reg.into(), value});
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let reg = "a";
        let value = "3";
        assert_eq!(parse(reg, value), Box::new(Set {register: "a".into(), value:  Value::Literal(3)}));

        let reg = "a";
        let value = "-3";
        assert_eq!(parse(reg, value), Box::new(Set {register: "a".into(), value:  Value::Literal(-3)}));

        let reg = "a";
        let value = "b";
        assert_eq!(parse(reg, value), Box::new(Set {register: "a".into(), value:  Value::Register("b".into())}));
    }

    #[test]
    fn test_execute() {
        let mut environment = Environment::new();
        let instruction = Set {register: "a".into(), value:  Value::Literal(5)};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 5);
    }

    #[test]
    fn test_execute_register() {
        let mut environment = Environment::new();
        environment.set(&"a", 2);
        environment.set(&"b", 3);
        let instruction = Set {register: "a".into(), value: Value::Register("b".into())};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 3);
        assert_eq!(environment.get(&"b"), 3);
    }
}