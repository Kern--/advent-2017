#![feature(try_from)]


#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod processor;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod util;

use std::io::{self, Read};
use std::convert::TryFrom;
use std::error::Error;
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
  advent-2017 tower [<input>]
  advent-2017 interpret [<input>]
  advent-2017 stream [<input>]
  advent-2017 knothash <variant> [<input>]
  advent-2017 hexgrid [<input>]
  advent-2017 pipes [<input>]
  advent-2017 firewall [<input>]
  advent-2017 defragment [<input>]
  advent-2017 generator <a> <b> [<aalignment>] [<balignment>] [<trials>],
  advent-2017 dance <repetitions> [<input>]
  advent-2017 spinlock [<input>]
  advent-2017 duet [<input>]
  advent-2017 route [<input>]
  advent-2017 particles [<input>]
  advent-2017 enhance <trials> [<input>]
  advent-2017 virus <trials> <variant> [<input>]
  advent-2017 coprocessor <variant> [<input>]
  advent-2017 bridge [<input>]
";

#[derive(Debug, Deserialize, PartialEq)]
enum Variant {
    Simple,
    Complex
}

#[derive(Debug, Deserialize)]
struct Args {
    arg_input: Option<String>,
    arg_variant: Option<Variant>,
    arg_a: u64,
    arg_b: u64,
    arg_aalignment: Option<u64>,
    arg_balignment: Option<u64>,
    arg_trials: Option<u32>,
    arg_repetitions: u32,
    cmd_captcha: bool,
    cmd_checksum: bool,
    cmd_spiralmemory: bool,
    cmd_stress: bool,
    cmd_passphrase: bool,
    cmd_maze: bool,
    cmd_memory: bool,
    cmd_tower: bool,
    cmd_interpret: bool,
    cmd_stream: bool,
    cmd_knothash: bool,
    cmd_hexgrid: bool,
    cmd_pipes: bool,
    cmd_firewall: bool,
    cmd_defragment: bool,
    cmd_generator: bool,
    cmd_dance: bool,
    cmd_spinlock: bool,
    cmd_duet: bool,
    cmd_route: bool,
    cmd_particles: bool,
    cmd_enhance: bool,
    cmd_virus: bool,
    cmd_coprocessor: bool,
    cmd_bridge: bool,
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
    } else if args.cmd_tower {
        let input = args.get_input();
        let tower = day7::Tower::from_str(&input);
        if let Some(ref tower) = tower {
            println!("base: {}, corrected_weigth: {}", tower.base, tower.calculate_corrected_weight());
            return;
        }
        println!("Could not parse tower");
    } else if args.cmd_interpret {
        let input = args.get_input();
        let mut interpreter = day8::Interpreter::from_str(&input).unwrap();
        interpreter.execute();
        println!("current largest: {}, largest ever: {}", interpreter.get_current_largest_value(), interpreter.get_largest_value());
    } else if args.cmd_stream {
        let input = args.get_input();
        let group = day9::Group::from_str(&input);
        if let Some(group) = group {
            println!("score: {}, garbage: {}", group.compute_total_score(), group.compute_total_garbage());
            return;
        }
        println!("Could not parse stream");
    } else if args.cmd_knothash {
        let input = args.get_input();
        match args.arg_variant.unwrap() {
            Variant::Simple => {
                let data = input.split(",").map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();
                let mut knot = day10::Knot::new(255);
                knot.compute_round(&data);
                println!("{}", knot.compute_fingerprint());
            },
            Variant::Complex => {
                let data = input.into_bytes();
                let mut knot = day10::Knot::new(255);
                println!("{}", util::to_hex_string(&knot.compute_hash(&data)));
            }
        }
        
    } else if args.cmd_hexgrid {
        let input = args.get_input();
        let (current_distance, max_distance) = day11::compute_distance(&input);
        println!("current distance: {}, max_distance: {}", current_distance, max_distance);
    } else if args.cmd_pipes {
        let input = args.get_input();
        let graph = day12::parse_graph(&input);
        if let Some(graph) = graph {
            println!("#elements in group containing 0: {}, num groups {}", day12::find_group(0, &graph).len(), day12::find_all_groups(&graph).len());
            return;
        }
        println!("Could not parse graph");
    } else if args.cmd_firewall {
        let input = args.get_input();
        let firewall = day13::Firewall::from_str(&input);
        match firewall {
            Ok(firewall) => println!("severity: {}, min delay: {}", firewall.compute_severity(0), firewall.compute_min_safe_delay()),
            Err(error) => println!("{}", error)
        }
    } else if args.cmd_defragment {
        let input = args.get_input();
        println!("used squares: {}, num groups: {}", day14::count_used_squares(&input), day14::get_groups(&input).len());
    } else if args.cmd_generator {
        let a = day15::Generator::new(16807, args.arg_a, args.arg_aalignment);
        let b = day15::Generator::new(48271, args.arg_b, args.arg_balignment);
        let mut judge = day15::Judge::new(a, b);
        if let Some(trials) = args.arg_trials {
            println!("{}", judge.judge_trials(trials));
            return;
        }
        println!("{}", judge.judge());
    } else if args.cmd_dance {
        let input = args.get_input();
        let mut dance = day16::Dance::new();
        dance.dance_repeatedly(&input, args.arg_repetitions);
        println!("{}", dance);
    } else if args.cmd_spinlock {
        let input = args.get_input().parse::<u32>();
        match input {
            Ok(step_size) => {
                let mut spinlock = day17::SpinLock::new(step_size);
                let mut pseudospinlock = day17::PseudoSpinLock::new(step_size);
                println!("spinlock: {}, angry spinlock: {}", spinlock.short_circuit(2017), pseudospinlock.short_circuit(50_000_000));
            },
            Err(error) => println!("Could not parse input, {}", error)
        }
    } else if args.cmd_duet {
        let input = args.get_input();
        let interpreter = day18::SoundInterpreter::new(&input);
        if let Some(mut interpreter) = interpreter {
            println!("{}", interpreter.execute());
            return;
        }
        println!("Could not parse input");
    } else if args.cmd_route {
        let input = args.get_input();
        let diagram = day19::Diagram::parse(&input);
        let (result, steps) = diagram.navigate();
        println!("result: {}, steps: {}", result, steps);
    } else if args.cmd_particles {
        let input = args.get_input();
        println!("without collision: {}, remaining after collision: {}", day20::simulate(&input), day20::simulate_with_collision(&input));
    } else if args.cmd_enhance {
        let input = args.get_input();
        let mut matrix = day21::matrix::Matrix::try_from(".#./..#/###").unwrap();
        let rulebook = day21::rulebook::RuleBook::try_from(input.as_str());
        let rounds = args.arg_trials.unwrap();
        match rulebook {
            Ok(rulebook) => {
                for _ in 0..rounds {
                    let enhanced_matrix = rulebook.enhance(matrix);
                    match enhanced_matrix {
                        Err(error) => { println!("{}", error.description()); return; },
                        Ok(enhanced_matrix) => matrix = enhanced_matrix
                    };
                }
                println!("{}", matrix.num_on_pixels());
            },
            Err(error) => println!("{}", error.description())
        }
    } else if args.cmd_virus {
        let input = args.get_input();
        let grid = day22::Grid::try_from(&input);
        let resistant =  args.arg_variant == Some(Variant::Complex);
        match grid {
            Ok(grid) => {
                let bursts = args.arg_trials.unwrap_or(10000) as usize;
                let mut virus = day22::Virus::new(grid, resistant);
                println!("{}", virus.run(bursts));
            },
            Err(error) => {
                println!("error parsing grid: {}", error.description());
            }
        }
    } else if args.cmd_coprocessor {
        let input = args.get_input();
        match args.arg_variant.unwrap_or(Variant::Simple) {
            Variant::Simple => {
                let mut coprocessor = day23::Coprocessor::new(&input);
                if let Some(ref mut coprocessor) = coprocessor {
                    println!("{}", coprocessor.execute());
                    return;
                }
                println!("Could not parse instructions");
            },
            Variant::Complex => {
                let parsed = input.parse::<usize>();
                if let Ok(parsed) = parsed {
                    println!("{}", day23::calculate_non_primes(parsed));
                    return;
                }
                println!("Input must be a positive integer");
            }
        }   
    } else if args.cmd_bridge {
        let input = args.get_input();
        let strongest_bridge = day24::compute_strongest_bridge(&input);
        match strongest_bridge {
            Ok(strength) => println!("{}", strength),
            Err(error) => println!("{}", error)
        }
    }
}
