#[macro_use]
extern crate serde_derive;
extern crate docopt;

pub mod day1;
pub mod util;

use docopt::Docopt;

const USAGE: &'static str = "
advent-2017

Usage:
  advent-2017 captcha <variant> <input>

";

#[derive(Debug, Deserialize)]
enum Variant {
    Simple,
    Complex
}

#[derive(Debug, Deserialize)]
struct Args {
    arg_input: String,
    arg_variant: Variant,
    cmd_captcha: bool
}



fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());

    if args.cmd_captcha {
        let input = util::string_to_number_slice(&args.arg_input);
        if let Some(input) = input {
            match args.arg_variant {
                Variant::Simple => println!("{}", day1::simple_captcha(&input[..])),
                Variant::Complex => println!("{}", day1::complex_captcha(&input[..]))
            }
            return;
        }
        println!("input must be a string of digits")
    }
}
