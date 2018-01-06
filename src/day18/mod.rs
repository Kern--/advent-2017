mod instruction;

use super::processor::{Environment, SpecialRegister};
use super::processor::Value;
use self::instruction::{Instruction, InstructionType};

pub struct SoundInterpreter {
    environment: Environment,
    instructions: Vec<Box<Instruction>>
}

/// An interpreter that can parse and execute a series of instructions 
impl SoundInterpreter {
    pub fn new(input: &str) -> Option<SoundInterpreter> {
        let instructions = input.split("\n").map(instruction::parse).collect::<Option<Vec<Box<Instruction>>>>();
        if let Some(instructions) = instructions {
            return Some(SoundInterpreter {environment: Environment::new(), instructions});
        }
        None
    }

    /// Executes the interpreter
    pub fn execute(&mut self) -> i64 {
        let mut halt = false;
        let mut value = 0;
        while !halt {
            let pc = self.environment.get_pc();
            // Make sure we're still in bounds, else die
            if pc < 0 || pc > self.instructions.len() as i64 {
                break;
            }
            // Update the PC
            self.environment.step_pc();
            // Execute the instruction at pc
            let instruction = &self.instructions[pc as usize];
            let result = instruction.execute(&mut self.environment);
            // Check for halt condition (recovered a value)
            if instruction.get_type() == InstructionType::Rcv {
                if let Some(sound) = result {
                    value = sound;
                    halt = true;
                }
            }
        }
        value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpreter() {
        let input = "set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2";
        let mut interpreter = SoundInterpreter::new(input).unwrap();
        assert_eq!(interpreter.execute(), 4);
    }
}