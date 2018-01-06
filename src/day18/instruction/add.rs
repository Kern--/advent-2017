use super::{InstructionType, Instruction};
use super::super::Environment;
use super::super::Value;

#[derive(Debug, PartialEq, Eq)]
pub struct Add {
    register: String,
    value: Value
}

impl Instruction for Add {
    fn get_type(&self) -> InstructionType {
        InstructionType::Add
    }

    fn execute(&self, environment: &mut Environment) -> Option<i64> {
        let old_value = environment.get(&self.register);
        let addition = environment.get_value(&self.value);
        let new_value = old_value + addition;
        environment.set(&self.register, new_value);
        Some(new_value)
    }
}

pub fn parse(reg: &str, val: &str) -> Box<Add>
    where Add: Instruction {
    let value = Value::parse(val);
    return Box::new(Add {register: reg.into(), value});
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let reg = "a";
        let value = "3";
        assert_eq!(parse(reg, value), Box::new(Add {register: "a".into(), value:  Value::Literal(3)}));

        let reg = "a";
        let value = "-3";
        assert_eq!(parse(reg, value), Box::new(Add {register: "a".into(), value:  Value::Literal(-3)}));

        let reg = "a";
        let value = "b";
        assert_eq!(parse(reg, value), Box::new(Add {register: "a".into(), value:  Value::Register("b".into())}));
    }

    #[test]
    fn test_execute() {
        let mut environment = Environment::new();
        let instruction = Add {register: "a".into(), value:  Value::Literal(-3)};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), -3);
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), -6);
    }

    #[test]
    fn test_execute_register() {
        let mut environment = Environment::new();
        environment.set(&"a", 2);
        environment.set(&"b", 3);
        let instruction = Add {register: "a".into(), value: Value::Register("b".into())};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 5);
        assert_eq!(environment.get(&"b"), 3);
    }
}