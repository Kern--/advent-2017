use super::environment::Environment;

mod snd;
mod set;
mod add;
mod mul;
mod rem;
mod rcv;
mod jgz;

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionType {
    Snd,
    Set,
    Add,
    Mul,
    Mod,
    Rcv,
    Jgz
}

/// An instruction in the instruction set
pub trait Instruction {
    fn get_type(&self) -> InstructionType;
    fn execute(&self, &mut Environment) -> Option<i64>;
}

pub fn parse(input: &str) -> Option<Box<Instruction>> {
    let parts = input.split(" ").collect::<Vec<&str>>();
    match parts[0] {
        "snd" => Some(snd::parse(parts[1])),
        "set" => Some(set::parse(parts[1], parts[2])),
        "add" => Some(add::parse(parts[1], parts[2])),
        "mul" => Some(mul::parse(parts[1], parts[2])),
        "mod" => Some(rem::parse(parts[1], parts[2])),
        "rcv" => Some(rcv::parse(parts[1])),
        "jgz" => Some(jgz::parse(parts[1], parts[2])),
        _ => None
    }
}