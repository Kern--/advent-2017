#[macro_use]
extern crate serde_derive;
extern crate docopt;

pub mod day1;
pub mod day2;
pub mod util;

use std::io::{self, Read};
use docopt::Docopt;

const USAGE: &'static str = "
advent-2017

Usage:
  advent-2017 captcha <variant> [<input>]
  advent-2017 checksum <variant> [<input>]
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
    cmd_checksum: bool
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
    }
}
