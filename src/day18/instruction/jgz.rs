use super::{InstructionType, Instruction};
use super::super::Environment;
use super::super::Value;

#[derive(Debug, PartialEq, Eq)]
pub struct Jgz {
    condition: Value,
    value: Value
}

impl Instruction for Jgz {
    fn get_type(&self) -> InstructionType {
        InstructionType::Jgz
    }

    fn execute(&self, environment: &mut Environment) -> Option<i64> {
        let reg_value = environment.get_value(&self.condition);
        if reg_value > 0 {
            let offset = environment.get_value(&self.value);
            environment.jump_pc(offset);
            return Some(offset);
        }
        None
    }
}

pub fn parse(reg: &str, val: &str) -> Box<Jgz> {
    let condition = Value::parse(reg);
    let value = Value::parse(val);
    return Box::new(Jgz {condition, value});
}

#[cfg(test)]
mod test {
    use super::*;
    use processor::SpecialRegister;

    #[test]
    fn test_parse() {
        let reg = "a";
        let value = "3";
        assert_eq!(parse(reg, value), Box::new(Jgz {condition: Value::Register("a".into()), value:  Value::Literal(3)}));

        let reg = "a";
        let value = "b";
        assert_eq!(parse(reg, value), Box::new(Jgz {condition: Value::Register("a".into()), value:  Value::Register("b".into())}));
    }

    #[test]
    fn test_execute() {
        let mut environment = Environment::new();
        // The environment assumes PC has been updated before running the current instruction
        environment.step_pc();
        let instruction = Jgz { condition: Value::Register("a".into()), value: Value::Literal(2) };
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
        let instruction = Jgz { condition: Value::Register("a".into()), value: Value::Register("b".into()) };
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
