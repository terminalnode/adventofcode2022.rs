use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::process::exit;
use ansi_term::Color::Red;
use getopts::{Matches, Fail, Options};

pub struct Day01 { file_name: String }

impl Day01 {
    fn get_counts(&self) -> Result<Vec<i32>, String> {
        let lines: Vec<String> = match self.read_file_as_lines() {
            Ok(file) => file,
            Err(e) => return Err(e.to_string()),
        };

        let counts: Vec<i32> = lines.split(|it| it == "")
            .map(|it| {
                it.iter().map(|it| it.parse::<i32>().unwrap()).sum::<i32>()
            }).collect();
        Ok(counts)
    }
}

impl Day for Day01 {
    fn new(file_name: String) -> Self { Day01 { file_name } }

    fn get_file_name(&self) -> String {
        self.file_name.clone()
    }

    fn part1(&self) -> Result<String, String> {
        let counts = match self.get_counts() {
            Ok(ns) => ns,
            Err(e) => return Err(e),
        };
        match counts.iter().max() {
            None => Err("There were no values in the list!".to_string()),
            Some(n) => Ok(format!("Highest value is {}", n)),
        }
    }

    fn part2(&self) -> Result<String, String> {
        let mut counts = match self.get_counts() {
            Ok(ns) => ns,
            Err(e) => return Err(e),
        };
        counts.sort_by(|a, b| b.cmp(a));
        let sum: i32 = counts.iter().take(3).sum();
        Ok(format!("Sum of top three highest is {}", sum))
    }
}

pub trait Day {
    fn new(file_name: String) -> Self;
    fn get_file_name(&self) -> String;

    fn run(&self, part: String) -> Result<String, String> {
        match part.as_str() {
            "1" => self.part1(),
            "2" => self.part2(),
            "all" => Err("I have not yet been arsed to implement 'all'.".to_string()),
            _ => return Err(format!("There is no part {}", part)),
        }
    }

    fn part1(&self) -> Result<String, String> {
        Err("Not implemented yet!".to_string())
    }

    fn part2(&self) -> Result<String, String> {
        Err("Not implemented yet!".to_string())
    }

    fn read_file(&self) -> Result<BufReader<File>, Error> {
        match File::open(self.get_file_name()) {
            Ok(f) => Ok(BufReader::new(f)),
            Err(e) => Err(e),
        }
    }

    fn read_file_as_lines(&self) -> Result<Vec<String>, Error> {
        self.read_file()?.lines().collect()
    }
}

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
