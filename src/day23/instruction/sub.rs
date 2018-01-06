use super::{InstructionType, Instruction, Environment, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct Sub {
    register: String,
    value: Value
}

impl Instruction for Sub {
    fn get_type(&self) -> InstructionType {
        InstructionType::Sub
    }

    fn execute(&self, environment: &mut Environment) -> Option<i64> {
        let old_value = environment.get(&self.register);
        let subtraction = environment.get_value(&self.value);
        let new_value = old_value - subtraction;
        environment.set(&self.register, new_value);
        Some(new_value)
    }
}

pub fn parse(reg: &str, val: &str) -> Box<Sub>
    where Sub: Instruction {
    let value = Value::parse(val);
    return Box::new(Sub {register: reg.into(), value});
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let reg = "a";
        let value = "3";
        assert_eq!(parse(reg, value), Box::new(Sub {register: "a".into(), value:  Value::Literal(3)}));

        let reg = "a";
        let value = "-3";
        assert_eq!(parse(reg, value), Box::new(Sub {register: "a".into(), value:  Value::Literal(-3)}));

        let reg = "a";
        let value = "b";
        assert_eq!(parse(reg, value), Box::new(Sub {register: "a".into(), value:  Value::Register("b".into())}));
    }

    #[test]
    fn test_execute() {
        let mut environment = Environment::new();
        let instruction = Sub {register: "a".into(), value:  Value::Literal(-3)};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 3);
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 6);
    }

    #[test]
    fn test_execute_register() {
        let mut environment = Environment::new();
        environment.set(&"a", 2);
        environment.set(&"b", 3);
        let instruction = Sub {register: "a".into(), value: Value::Register("b".into())};
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), -1);
        assert_eq!(environment.get(&"b"), 3);
    }
}