use std::env;
use std::process::exit;
use ansi_term::Color::Red;
use getopts::{Matches, Fail, Options};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("d", "day", "specify which day to run", "[1..24]");
    opts.optopt("p", "part", "specify which part to run", "[1|2|all]");
    opts.optflag("h", "help", "print this help menu");

    match opts.parse(&args[1..]) {
        Ok(m) => run_with_opts(m, &program, &opts),
        Err(f) => print_error(&program, &opts, f),
    };
}

fn run_with_opts(
    m: Matches,
    program: &String,
    opts: &Options,
) {
    if m.opt_present("h") {
        print_usage(&program, &opts);
        return
    }

    let day = match m.opt_str("day") {
        None => { abort_missing_opt("day".to_string(), &program, &opts); return },
        Some(n) => match n.parse::<i32>() {
            Ok(n) => n,
            Err(_) => { abort_missing_opt("day".to_string(), &program, &opts); return },
        }
    };
    let part = m.opt_str("part").unwrap_or("all".to_string());

    run_day(day, part);
}

fn run_day(
    day: i32,
    part: String,
) {
    println!("Now I would run day {} part '{}' if I could :)", day, part);
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

fn print_usage(
    program: &str,
    opts: &Options,
) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));
}
