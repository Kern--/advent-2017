use super::super::processor::{Environment, Value};
use std::fmt::Debug;

mod jnz;
mod set;
mod mul;
mod sub;

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionType {
    Set,
    Mul,
    Sub,
    Jnz
}

/// An instruction in the instruction set
pub trait Instruction: Debug {
    fn get_type(&self) -> InstructionType;
    fn execute(&self, &mut Environment) -> Option<i64>;
}

pub fn parse(input: &str) -> Option<Box<Instruction>> {
    let parts = input.split(" ").collect::<Vec<&str>>();
    match parts[0] {
        "set" => Some(set::parse(parts[1], parts[2])),
        "mul" => Some(mul::parse(parts[1], parts[2])),
        "sub" => Some(sub::parse(parts[1], parts[2])),
        "jnz" => Some(jnz::parse(parts[1], parts[2])),
        _ => None
    }
}