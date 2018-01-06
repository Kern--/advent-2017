mod instruction;

use super::processor::Environment;
use self::instruction::{Instruction, InstructionType};

pub struct Coprocessor {
    environment: Environment,
    instructions: Vec<Box<Instruction>>
}

/// An interpreter that can parse and execute a series of instructions 
impl Coprocessor {
    pub fn new(input: &str) -> Option<Coprocessor> {
        let instructions = input.split("\n").map(instruction::parse).collect::<Option<Vec<Box<Instruction>>>>();
        if let Some(instructions) = instructions {
            return Some(Coprocessor {environment: Environment::new(), instructions});
        }
        None
    }

    /// Executes the interpreter
    pub fn execute(&mut self) -> i64 {
        let mut value = 0;
        loop {
            let pc = self.environment.get_pc();
            // Make sure we're still in bounds, else die
            if pc < 0 || pc >= self.instructions.len() as i64 {
                break;
            }
            // Get the instruction at pc
            let instruction = &self.instructions[pc as usize];
            // If it's a mul, update the value
            if instruction.get_type() == InstructionType::Mul {
                value += 1;
            }
            // Update the PC
            self.environment.step_pc();
            // Execute the instruction
            instruction.execute(&mut self.environment);
            
        }
        value
    }
}

/// Calculates the number of non-prime numbers
/// bettween [b * 100 + 100_000, b * 100 + 117_000]
/// This is a translation of the input assembly
pub fn calculate_non_primes(b: usize) -> usize {
    let mut current = b * 100 + 100_000;
    let end = current + 17_000;
    let mut result = 0;
    while current <= end {
        for d in 2..current - 1 {
            if current % d == 0 {
                // not prime. 
                result += 1;
                break;
            }
        }
        current += 17; 
    }
    result
}