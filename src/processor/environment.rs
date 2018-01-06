use std::collections::HashMap;
use super::value::Value;

pub struct Environment {
    state: HashMap<String, i64>

}

/// A register that has special meaning
pub enum SpecialRegister {
    /// The program counter
    PC,
}

impl SpecialRegister {
    pub fn get_name(&self) -> &'static str {
        match *self {
            SpecialRegister::PC => "pc",
        }
    }
}

/// A store of registers and their values
impl Environment {
    pub fn new() -> Environment {
        Environment { state: HashMap::new() }
    }

    /// Gets the current value of a register
    pub fn get<T>(&mut self, register: &T) -> i64
        where T: Into<String> + Clone {
        let entry = self.state.entry(register.clone().into()).or_insert(0);
        *entry
    }

    /// Gets the current value of a `Value` which may be a literal or a register
    pub fn get_value(&mut self, value: &Value) -> i64 {
        match value {
            &Value::Literal(value) => value,
            &Value::Register(ref name) => self.get(name)
        }
    }

    /// Sets the value of a register
    pub fn set<T>(&mut self, register: &T, value: i64)
        where T: Into<String> + Clone {
        let entry = self.state.entry(register.clone().into()).or_insert(0);
        *entry = value;
    }

    /// Gets the current PC value (convenience method, the same could be accomplished with `get` and `SpecialRegister`)
    pub fn get_pc(&mut self) -> i64 {
        let pc = SpecialRegister::PC.get_name();
        self.get(&pc)
    }

    /// Increments the current PC value by 1 (convenience method, the same could be accomplished with `set` and `SpecialRegister`)
    pub fn step_pc(&mut self) {
        self.jump_pc(1 + 1);
    }

    /// Increments the current PC value by an `offset` - 1
    /// This method assumes the pc has already stepped before being called and thus the PC
    ///  is off by 1 from the original value (i.e. the environment has a delay slot even though the instruction set does not)
    ///  (convenience method, the same could be accomplished with `get` and `SpecialRegister`)
    pub fn jump_pc(&mut self, offset: i64) {
        let pc = SpecialRegister::PC.get_name();
        let old_value = self.get(&pc);
        self.set(&pc, old_value + offset - 1);
    }
}