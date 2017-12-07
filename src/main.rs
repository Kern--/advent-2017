#[macro_use]
extern crate serde_derive;
extern crate docopt;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod util;

use std::io::{self, Read};
use docopt::Docopt;

const USAGE: &'static str = "
advent-2017

Usage:
  advent-2017 captcha <variant> [<input>]
  advent-2017 checksum <variant> [<input>]
  advent-2017 spiralmemory [<input>]
  advent-2017 spiralmemory stress [<input>]
  advent-2017 passphrase <variant> [<input>]
  advent-2017 maze <variant> [<input>]
  advent-2017 memory redistribute <variant> [<input>]
";

#[derive(Debug, Deserialize)]
enum Variant {
    Simple,
    Complex
}

#[derive(Debug, Deserialize)]
struct Args {
    arg_input: Option<String>,
    arg_variant: Option<Variant>,
    cmd_captcha: bool,
    cmd_checksum: bool,
    cmd_spiralmemory: bool,
    cmd_stress: bool,
    cmd_passphrase: bool,
    cmd_maze: bool,
    cmd_memory: bool,
}

impl Args {
    fn get_input(&self) -> String {
        if let Some(input) = self.arg_input.clone() {
            return input
        }
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    }
}


fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());

    if args.cmd_captcha {
        let input = util::string_to_number_slice(&args.get_input());
        if let Some(input) = input {
            match args.arg_variant.unwrap() {
                Variant::Simple => println!("{}", day1::simple_captcha(&input[..])),
                Variant::Complex => println!("{}", day1::complex_captcha(&input[..]))
            }
            return;
        }
        println!("input must be a string of digits")
    } else if args.cmd_checksum {
        let input = util::string_to_number_table(&args.get_input());
        if let Some(input) = input {
            match args.arg_variant.unwrap() {
                Variant::Simple => println!("{}", day2::compute_simple_checksum(&input)),
                Variant::Complex => println!("{}", day2::compute_complex_checksum(&input))
            }
            return;
        }
        println!("input must be a number table")
    } else if args.cmd_spiralmemory {
        let input = args.get_input().parse::<u32>().unwrap();
        if args.cmd_stress {
            println!("{}", day3::run_stress_test(input));
        } else {
            println!("{}", day3::compute_memory_steps(input));
        }
    } else if args.cmd_passphrase {
        let input = args.get_input();

        let parsed_input = input.split("\n").map(|s| s.split(" "));
        match args.arg_variant.unwrap() {
            Variant::Simple => println!("{}", day4::compute_num_valid_simple_passphrases(parsed_input)),
            Variant::Complex => println!("{}", day4::compute_num_valid_complex_passphrases(parsed_input)),
        }
    } else if args.cmd_maze {
        let input = args.get_input().split("\n").map(|s| s.parse::<i32>().ok()).collect::<Option<Vec<i32>>>();
        if let Some(mut input) = input {
            match args.arg_variant.unwrap() {
                Variant::Simple => println!("{}", day5::compute_steps_to_exit_simple_maze(&mut input)),
                Variant::Complex => println!("{}", day5::compute_steps_to_exit_complex_maze(&mut input)),
            }
            return;
        }
        println!("input must be numbers separated by a new line")
    } else if args.cmd_memory {
        let mut input = util::separated_string_to_number_slice(&args.get_input(), "\t");
        if let Some(ref mut input) = input {
            match args.arg_variant.unwrap() {
                Variant::Simple => println!("{}", day6::detect_simple_redistribution_loop(input)),
                Variant::Complex => println!("{}", day6::detect_complex_redistribution_loop(input)),
            }
            return;
        }
        println!("input must be numbers separated by \\t");
    }
}
