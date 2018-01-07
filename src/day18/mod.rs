mod instruction;

use super::processor::{Environment, Value};
use self::instruction::{Instruction, InstructionType};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

struct Program {
    environment: Rc<RefCell<Environment>>,
    instructions: Rc<Vec<Box<Instruction>>>,
    instruction_count: HashMap<InstructionType, usize>
}

impl Program {
    fn step(&mut self) -> (InstructionType, Option<i64>) {
        // Add a scope so that the immutable borrow via env/instruction
        //  do not prevent us from incrementing the instruction count
        let (result, instruction_type) = 
        {
            let mut env = self.environment.borrow_mut();
            let pc = env.get_pc();
            // Make sure we're still in bounds, else die
            if pc < 0 || pc >= self.instructions.len() as i64 {
                return (InstructionType::Halt, None);
            }
            // Fetch the instruction at pc
            let instruction = &self.instructions[pc as usize];
            // Update the PC
            env.step_pc();
            // Execute the instruction at pc
            let result = instruction.execute(&mut env);
            (result, instruction.get_type())
        };

        // Log execution if it produced a value
        if result != None {
            self.increment_instruction_count(instruction_type.clone());
        }
        (instruction_type, result)
    }

    fn increment_instruction_count(&mut self, instruction_type: InstructionType) {
        let entry = self.instruction_count.entry(instruction_type).or_insert(0);
        *entry = *entry + 1;
    }

    fn instruction_count(&self, instruction_type: InstructionType) -> usize {
        *self.instruction_count.get(&instruction_type).unwrap_or(&0)
    }
}

pub struct Interpreter {
    program_zero: Program,
    program_one: Program,
}

/// An interpreter that can parse and execute a series of instructions 
impl Interpreter {
    pub fn new(input: &str) -> Option<Interpreter> {
        let instructions = input.split("\n").map(instruction::parse).collect::<Option<Vec<Box<Instruction>>>>();
        if let Some(instructions) = instructions {
            // Create a shared, immutable reference to the instructions
            let shared_instructions = Rc::new(instructions);
            // Create 2 environments
            let env_zero = Rc::new(RefCell::new(Environment::new()));
            let env_one = Rc::new(RefCell::new(Environment::new()));
            // link the environments
            env_zero.borrow_mut().link(env_one.clone());
            env_one.borrow_mut().link(env_zero.clone());
            // Setup the process id registers
            env_zero.borrow_mut().set(&"p", 0);
            env_one.borrow_mut().set(&"p", 1);

            let program_zero = Program { environment: env_zero, instructions: shared_instructions.clone(), instruction_count: HashMap::new() };
            let program_one = Program { environment: env_one, instructions: shared_instructions, instruction_count: HashMap::new() };
            return Some(Interpreter {program_zero, program_one });
        }
        None
    }

    /// Executes the interpreter
    pub fn execute(&mut self) -> usize {
        let mut made_progress = true;
        let mut program_zero_halted = false;
        let mut program_one_halted = false;
        // While not in deadlock
        while made_progress {
            // Reset progress (might be in dead lock)
            made_progress = false;
            while !program_zero_halted {
                let (instruction_type, value) = self.program_zero.step();
                // If dead, exit
                if instruction_type == InstructionType::Halt {
                    program_zero_halted = true;
                    break;
                }
                // If blocking receive, exit
                if instruction_type == InstructionType::Rcv && value == None {
                    break;
                }
                // If we made it here, we did some work.
                // Therefore we're not in deadlock (yet)
                made_progress = true;
            }
            while !program_one_halted {
                let (instruction_type, value) = self.program_one.step();
                // If dead, exit
                if instruction_type == InstructionType::Halt {
                    program_one_halted = true;
                    break;
                }
                // If blocking receive, exit
                if instruction_type == InstructionType::Rcv && value == None {
                    break;
                }
                // If we made it here, we did some work.
                // Therefore we're not in deadlock (yet)
                made_progress = true;
            }
        }
        self.program_one.instruction_count(InstructionType::Snd)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpreter() {
        let input = "set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2";
        let mut interpreter = Interpreter::new(input).unwrap();
        assert_eq!(interpreter.execute(), 1);
    }
}