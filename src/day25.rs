use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::convert::TryFrom;
use regex::Regex;

// A series of regexes for parsing the input
lazy_static!(
    static ref START_REGEX: Regex = Regex::new(r"Begin in state (\pL)\.").unwrap();
    static ref STEPS_REGEX: Regex = Regex::new(r"Perform a diagnostic checksum after (\d+) steps\.").unwrap();
    static ref NAME_REGEX: Regex = Regex::new(r"In state (\pL):").unwrap();
    static ref CURRENT_VALUE_REGEX: Regex = Regex::new(r"  If the current value is (\d):").unwrap();
    static ref NEW_VALUE_REGEX: Regex = Regex::new(r"    - Write the value (\d)\.").unwrap();
    static ref DIRECTION_REGEX: Regex = Regex::new(r"    - Move one slot to the (\pL+)\.").unwrap();
    static ref NEW_STATE_REGEX: Regex = Regex::new(r"    - Continue with state (\pL)\.").unwrap();
);

#[derive(Debug, PartialEq, Eq)]
/// A rule for how to proceed when a turing_machine comes across `current_value` on its tape.
/// The `new_value` should be written to the tape,
/// The state should take one step in `step_direction`
/// And the turing machine should continue in state `new_state`
pub struct Rule {
    current_value: i64,
    new_value: i64,
    step_direction: String,
    new_state: String
}

#[derive(Debug, PartialEq, Eq)]
/// A State of a turing machine
pub struct State {
    name: String,
    /// The rules used to proceed one step in the turing machine
    rules: HashMap<i64, Rule>, 
}

impl State {
    fn update(&self, current_value: i64) -> (i64, String, String) {
        let rule = self.rules.get(&current_value);
        if let Some(rule) = rule {
            return (rule.new_value, rule.step_direction.clone(), rule.new_state.clone());
        }
        (0, String::from("left"), self.name.clone())
    }
}

/// Extracts a string from `input` using `regex`
///  Assumes the regex has exactly 1 capture group
///  Utility method to map regex maching to ParseErrors
fn extract_string<'a>(input: &'a str, regex: &Regex) -> Result<String, ParseError<'a>> {
    let captures = regex.captures(input);
    if let Some(captures) = captures {
        if captures.len() == 2 {
            return Ok(String::from(&captures[1]));
        }
    }
    return Err(ParseError::InvalidFormat(input));
}

/// Extracts a number from `input` using `regex`
///  Assumes the regex has exactly 1 capture group
///  Utility method to map regex maching to ParseErrors
fn extract_number<'a>(input: &'a str, regex: &Regex) -> Result<i64, ParseError<'a>> {
    let captures = regex.captures(input);
    if let Some(captures) = captures {
        if captures.len() == 2 {
            let capture = &captures[1];
            return capture.parse::<i64>().map_err(|_| ParseError::NotANumber(input, String::from(capture)));
        }
    }
    return Err(ParseError::InvalidFormat(input));
}

impl <'a> TryFrom<&'a str> for State {
    type Error = ParseError<'a>;
    fn try_from(input: &'a str) -> Result<State, Self::Error> {
        let parts = input.split("\n").collect::<Vec<&str>>();
        if parts.len() != 9 {
            return Err(ParseError::InvalidState(input));
        }
        let name = extract_string(parts[0], &*NAME_REGEX)?;
        let mut rules = HashMap::new();
        for i in 0..2 {
            let current_value = extract_number(parts[i*4 + 1], &*CURRENT_VALUE_REGEX)?;
            let new_value = extract_number(parts[i*4 + 2], &*NEW_VALUE_REGEX)?;
            let step_direction = extract_string(parts[i*4 + 3], &*DIRECTION_REGEX)?;
            let new_state = extract_string(parts[i*4 + 4], &*NEW_STATE_REGEX)?;
            rules.insert(current_value, Rule { current_value, new_value, step_direction, new_state});
        }
        Ok(State{name , rules})
    }
}

pub struct TuringMachine {
    current_state: String,
    checksum_after: usize,
    states: HashMap<String, State>,
    tape: HashSet<i64>
}

impl TuringMachine {
    pub fn checksum(&self) -> usize {
        self.tape.len()
    }

    /// Runs the turing machine for as long as the input requests.
    /// returns the checksum after `checksum_after` steps
    pub fn run(&mut self) -> usize {
        let mut position = 0;
        for _ in 0..self.checksum_after {
            let current_value = if self.tape.contains(&position) { 1 } else { 0 };
            let state = self.states.get(&self.current_state);
            if let Some(state) = state {
                let (new_value, step_direction, new_state) = state.update(current_value);
                match new_value {
                    0 => self.tape.remove(&position),
                    _ => self.tape.insert(position)
                };
                match step_direction.as_str() {
                    "left" => position -= 1,
                    _ => position += 1
                }
                self.current_state = new_state;
            } else {
                break;
            }
        }
        self.checksum()
    }
}

impl <'a> TryFrom<&'a str> for TuringMachine {
    type Error = ParseError<'a>;
    fn try_from(input: &'a str) -> Result<TuringMachine, Self::Error> {
        // Related input is separates by "\n\n"
        let parts = input.split("\n\n").collect::<Vec<&str>>();
        // The initial state of the turing machine is in the first group
        let init = parts[0].split("\n").collect::<Vec<&str>>();
        if init.len() != 2 {
            return Err(ParseError::InvalidInit(parts[0]));
        }
        let current_state = extract_string(init[0], &*START_REGEX)?;
        let checksum_after = extract_number(init[1], &*STEPS_REGEX)? as usize;
        // Parse the states
        let states = parts.into_iter().skip(1).map(State::try_from).collect::<Result<Vec<State>, ParseError<'a>>>()?;
        // Convert states into a HashMap for faster lookup
        let mut states_map = HashMap::new();
        for state in states.into_iter() {
            states_map.insert(state.name.clone(), state);
        }
        Ok(TuringMachine { current_state, checksum_after, states: states_map, tape: HashSet::new() })
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    InvalidInit(&'a str),
    InvalidState(&'a str),
    InvalidFormat(&'a str),
    NotANumber(&'a str, String),
}

impl <'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            ParseError::InvalidInit(input) => write!(f, "Could not determine initial state from {}", input),
            ParseError::InvalidState(input) => write!(f, "Could not parse state: {}", input),
            ParseError::InvalidFormat(input) => write!(f, "Could not match regex against: {}", input),
            ParseError::NotANumber(input, ref nan) => write!(f, "Could not parse `{}` into a number in {}", nan, input)
        }
    }
}

impl <'a> Error for ParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            ParseError::InvalidInit(_) => "Invalid initial state",
            ParseError::InvalidState(_) => "Invalid state format",
            ParseError::InvalidFormat(_) => "Invalid input",
            ParseError::NotANumber(_, _) => "Not a number"
        }
    }
}