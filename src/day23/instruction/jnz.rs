use super::{InstructionType, Instruction, Environment, Value};

#[derive(Debug, PartialEq, Eq)]
pub struct Jnz {
    condition: Value,
    value: Value
}

impl Instruction for Jnz {
    fn get_type(&self) -> InstructionType {
        InstructionType::Jnz
    }

    fn execute(&self, environment: &mut Environment) -> Option<i64> {
        let condition = environment.get_value(&self.condition);
        if condition != 0 {
            let offset = environment.get_value(&self.value);
            environment.jump_pc(offset);
        }
        None
    }
}

pub fn parse(cond: &str, val: &str) -> Box<Jnz> {
    let value = Value::parse(val);
    let condition = Value::parse(cond);
    return Box::new(Jnz {condition, value});
}

#[cfg(test)]
mod test {
    use super::*;
    use processor::SpecialRegister;

    #[test]
    fn test_parse() {
        let reg = "a";
        let value = "3";
        assert_eq!(parse(reg, value), Box::new(Jnz {condition: Value::Register(reg.into()), value:  Value::Literal(3)}));

        let reg = "a";
        let value = "b";
        assert_eq!(parse(reg, value), Box::new(Jnz {condition: Value::Register(reg.into()), value:  Value::Register("b".into())}));
    }

    #[test]
    fn test_execute() {
        let mut environment = Environment::new();
        // The environment assumes PC has been updated before running the current instruction
        environment.step_pc();
        let instruction = Jnz { condition: Value::Register("a".into()), value: Value::Literal(2) };
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 0);
        assert_eq!(environment.get(&SpecialRegister::PC.get_name()),  1);

        environment.set(&"a", 1);
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 1);
        assert_eq!(environment.get(&SpecialRegister::PC.get_name()),  2);
    }

    #[test]
    fn test_execute_register() {
        let mut environment = Environment::new();
        // The environment assumes PC has been updated before running the current instruction
        environment.step_pc();
        let instruction = Jnz { condition: Value::Register("a".into()), value: Value::Register("b".into()) };
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 0);
        assert_eq!(environment.get(&"b"), 0);
        assert_eq!(environment.get(&SpecialRegister::PC.get_name()),  1);

        environment.set(&"a", 1);
        environment.set(&"b", 3);
        instruction.execute(&mut environment);
        assert_eq!(environment.get(&"a"), 1);
        assert_eq!(environment.get(&"b"), 3);
        assert_eq!(environment.get(&SpecialRegister::PC.get_name()),  3);
    }
}
