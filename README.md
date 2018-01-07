# Advent 2017
This repo contains my solutions to the [Advent of Code 2017](http://adventofcode.com/2017), written in Rust.


## Running
This repo requires a nightly rust compiler as it relies on [TryFrom](https://github.com/rust-lang/rust/issues/33417)

`cargo test` is the easiest way to check this code. Test coverage is decent across all challenges.

The individual solutions do not have any I/O, so it's expected that data comes from command line args or stdin, e.g.:
```
cat day18.data | cargo run duet
```

Solutions work for all challenges except day 18 part 1. The solution to part 2 was significantly different and so I opted to replace part 1 rather than extend it.

## Reflection
This repo served 2 personal purposes:
1) Learn Rust
2) Practice interesting programming problems

Importantly, this repo was not intended to be an example of a well designed system. There are many issues with my implementation both in code quality and overall architecture. I tried to make somewhat reasonable decisions when implementing the challenges, however when a part 2 broke some assumptions I made for part 1, I generally did the minimum to get part 2 to work rather than redesigning my algorithms.

If I were to go back and clean this repo up, my top priorities would be:
- Fix main.rs. Probably replace the cmds with an enum and split the main function into many smaller functions
- Use `Result` consistently. In earlier challenges, I was struggling enough with the borrow checker and didn't want the overhead of implementing a bunch of error types. As a result, many solutions use `Option` when `Result` is much more appropriate.
- Replace regexes with a proper parsing library. Many of my parsing mechanisms use regexes which are not very ergonomic.

