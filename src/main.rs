#[macro_use]
extern crate serde_derive;
extern crate docopt;

pub mod day1;
pub mod util;

use docopt::Docopt;

const USAGE: &'static str = "
advent-2017

Usage:
  advent-2017 captcha <input>

";

#[derive(Debug, Deserialize)]
struct Args {
    arg_input: String,
    cmd_captcha: bool
}



fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());

    if args.cmd_captcha {
        let input = util::string_to_number_slice(&args.arg_input);
        if let Some(input) = input {
            println!("{}", day1::captcha(&input[..]));
            return;
        }
        println!("input must be a string of digits")
    }
}
