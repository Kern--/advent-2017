use super::{InstructionType, Instruction};
use super::super::environment::Environment;
use super::super::value::Value;

#[derive(Debug, PartialEq, Eq)]
pub struct Mul {
    register: String,
    value: Value
}

impl Instruction for Mul {
    fn get_type(&self) -> InstructionType {
        InstructionType::Mul
    }

    fn execute(&self, environment: &mut Environment) -> Option<i64> {
        let old_value = environment.get(&self.register);
        let multiplier = environment.get_value(&self.value);
        let new_value = old_value * multiplier; 
        environment.set(&self.register, new_value);
        Some(new_value)
    }
}

pub fn parse(reg: &str, val: &str) -> Box<Mul> {
    let value = Value::parse(val);
    return Box::new(Mul {register: reg.into(), value});
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let reg = "a";
        let value = "3";
        assert_eq!(parse(reg, value), Box::new(Mul {register: "a".into(), value:  Value::Literal(3)}));

        let reg = "a";
        let value = "-3";
        assert_eq!(parse(reg, value), Box::new(Mul {register: "a".into(), value:  Value::Literal(-3)}));

        let reg = "a";
        let value = "b";
        assert_eq!(parse(reg, value), Box::new(Mul {register: "a".into(), value:  Value::Register("b".into())}));
    }

    #[test]
    fn test_execute() {
        let mut environment = Environment::new();
        environment.set(&"a", 1);
        let instruction = Mul {register: "a".into(), value:  Value::Literal(-3)};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), -3);
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 9);
    }

    #[test]
    fn test_execute_register() {
        let mut environment = Environment::new();
        environment.set(&"a", 2);
        environment.set(&"b", 3);
        let instruction = Mul {register: "a".into(), value: Value::Register("b".into())};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 6);
        assert_eq!(environment.get(&"b"), 3);
    }
}