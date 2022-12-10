extern crate core;

use std::env;
use std::process::exit;

use ansi_term::Color::Red;
use getopts::{Fail, Matches, Options};

use solution::{
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day10,
    Solution,
};

mod solution;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("d", "day", "specify which day to run", "[1..25]");
    opts.optopt("p", "part", "specify which part to run", "[1|2|all]");
    opts.optopt("f", "file", "specify file to use for input", "FILE");
    opts.optflag("h", "help", "print this help menu");

    match opts.parse(&args[1..]) {
        Ok(m) => println!("{}", run_with_opts(m, &program, &opts)),
        Err(f) => print_error(&program, &opts, f),
    };
}

// TODO this is a temporary solution while I figure out how to use this damn language
// region dirt
fn run_with_opts(
    m: Matches,
    program: &String,
    opts: &Options,
) -> String {
    if m.opt_present("h") {
        print_usage(&program, &opts);
        return get_usage(program, opts)
    }

    let day = match m.opt_str("day") {
        None => { abort_missing_opt("day".to_string(), &program, &opts); exit(1) },
        Some(n) => match n.parse::<i32>() {
            Ok(n) => n,
            Err(_) => { abort_missing_opt("day".to_string(), &program, &opts); exit(1) },
        }
    };
    let part = m.opt_str("part").unwrap_or("all".to_string());
    let file_name = match m.opt_str("file") {
        None => { abort_missing_opt("file".to_string(), &program, &opts); exit(1) },
        Some(n) => n,
    };

    run_day(day, part, file_name)
}

fn run_day(
    day: i32,
    part: String,
    file_name: String,
) -> String {
    let out = match day {
        1 => Day01::new(file_name).run(part.clone()),
        2 => Day02::new(file_name).run(part.clone()),
        3 => Day03::new(file_name).run(part.clone()),
        4 => Day04::new(file_name).run(part.clone()),
        5 => Day05::new(file_name).run(part.clone()),
        6 => Day06::new(file_name).run(part.clone()),
        7 => Day07::new(file_name).run(part.clone()),
        8 => Day08::new(file_name).run(part.clone()),
        // 9 => Day09::new(file_name).run(part.clone()),
        10 => Day10::new(file_name).run(part.clone()),
        _ => Err(format!("There is no day {}", day)),
    };

    match out {
        Ok(yay) => format!("Day {} #{}: {}", day, part, yay),
        Err(nay) => format!("Failed to run day {} #{}:\n{}", day, part, nay)
    }
}

fn abort_missing_opt(
    opt: String,
    program: &String,
    opts: &Options,
) {
    println!("{} Missing required option: {}", Red.paint("ERROR: "), opt);
    print_usage(program, opts);
    exit(1);
}

fn print_error(
    program: &String,
    opts: &Options,
    f: Fail,
) {
    println!("{} {}", Red.paint("ERROR:"), f);
    print_usage(program, opts);
    exit(1);
}

fn get_usage(
    program: &str,
    opts: &Options,
) -> String {
    let brief = format!("Usage: {} [options]", program);
    format!("{}", opts.usage(&brief))
}

fn print_usage(
    program: &str,
    opts: &Options,
) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));
}
// endregion dirt
