use itertools::Itertools;
use crate::solution::day10::Instruction::{ADDX, NOOP};
use crate::solution::Solution;

pub struct Day10 { file_name: String }

#[derive(Debug)]
enum Instruction {
    NOOP,
    ADDX(i32),
}

impl Day10 {
    fn parse(&self) -> Result<Vec<Instruction>, String> {
        self.read_file_as_lines()
            .or_else(|err| Err(format!("Failed to read file: {}", err)))?
            .iter().map(|s: &String| {
            let x = s.split(" ").collect::<Vec<&str>>();
            match x[0] {
                "addx" => {
                    x[1].parse::<i32>()
                        .or_else(|_| Err(format!("Failed to parse ADDX arg {}", x[1])))
                        .and_then(|it| Ok(ADDX(it)))
                },
                "noop" => Ok(NOOP),
                _ => Err("".to_string()),
            }
        }).collect::<Result<Vec<Instruction>, String>>()
    }
}

impl Solution for Day10 {
    fn new(file_name: String) -> Self { Day10 { file_name } }
    fn get_file_name(&self) -> String { return self.file_name.clone() }

    fn part1(&self) -> Result<String, String> {
        let instructions: Vec<Instruction> = self.parse()?;
        let mut cycles = vec![60, 100, 140, 180, 220].into_iter();

        let mut next_cycle: i32 = 20;
        let mut current_cycle: i32 = 0;
        let mut x: i32 = 1;
        let mut result: i32 = 0;
        for instruction in instructions.iter() {
            let (value, shift) = match instruction {
                NOOP => (0, 1),
                ADDX(n) => (n+0, 2),
            };

            if &current_cycle < &next_cycle && &current_cycle+&shift >= next_cycle {
                result += &next_cycle * &x;
                match cycles.next() {
                    None => break,
                    Some(n) => next_cycle = n+0,
                }
            }
            x += value;
            current_cycle += shift;
        }

        Ok(format!("Result {result}"))
    }

    fn part2(&self) -> Result<String, String> {
        let instructions: Vec<Instruction> = self.parse()?;

        let mut signals: Vec<i32> = Vec::with_capacity(240);
        let mut x: i32 = 1;
        {
            let mut current_cycle: i32 = 0;
            for instruction in instructions {
                let (value, shift) = match instruction {
                    NOOP => (0, 1),
                    ADDX(n) => (n + 0, 2),
                };

                (0..shift).for_each(|shift| {
                    let pos: usize = (current_cycle + shift) as usize;
                    signals.insert(pos, &x + 0);
                });
                current_cycle += shift;
                x += value;
            }
        }

        let monitor = (0..240)
            .map(|i| {
                let im40 = &i % 40;
                let min_value = (&signals[&i+0] - 1) as usize;
                let max_value = (&signals[&i+0] + 1) as usize;
                if im40 >= min_value && im40 <= max_value { '#' } else { '.' }
            }).collect::<Vec<char>>().chunks(40)
            .into_iter()
            .map(|x| { x.iter().join("") })
            .join("\n");
        Ok(format!("\n{}", monitor))
    }
}
