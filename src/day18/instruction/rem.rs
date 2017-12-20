use super::{InstructionType, Instruction};
use super::super::environment::Environment;
use super::super::value::Value;

#[derive(Debug, PartialEq, Eq)]
pub struct Rem {
    register: String,
    value: Value
}

impl Instruction for Rem {
    fn get_type(&self) -> InstructionType {
        InstructionType::Mod
    }

    fn execute(&self, environment: &mut Environment) -> Option<i64> {
        let old_value = environment.get(&self.register);
        let modulus = environment.get_value(&self.value); 
        let new_value = old_value % modulus;
        environment.set(&self.register, new_value);
        Some(new_value)
    }
}

pub fn parse(reg: &str, val: &str) -> Box<Rem> {
    let value = Value::parse(val);
    return Box::new(Rem {register: reg.into(), value});
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let reg = "a";
        let value = "3";
        assert_eq!(parse(reg, value), Box::new(Rem {register: "a".into(), value:  Value::Literal(3)}));

        let reg = "a";
        let value = "-3";
        assert_eq!(parse(reg, value), Box::new(Rem {register: "a".into(), value:  Value::Literal(-3)}));

        let reg = "a";
        let value = "b";
        assert_eq!(parse(reg, value), Box::new(Rem {register: "a".into(), value:  Value::Register("b".into())}));
    }

    #[test]
    fn test_execute() {
        let mut environment = Environment::new();
        environment.set(&"a", 8);
        let instruction = Rem {register: "a".into(), value:  Value::Literal(3)};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 2);
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 2);
    }

    #[test]
    fn test_execute_register() {
        let mut environment = Environment::new();
        environment.set(&"a", 8);
        environment.set(&"b", 3);
        let instruction = Rem {register: "a".into(), value: Value::Register("b".into())};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 2);
        assert_eq!(environment.get(&"b"), 3);
    }
}