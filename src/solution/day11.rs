use std::str::Lines;
use crate::solution::day11::Operation::{Add, Multiply, Square};
use crate::solution::day11::Test::Divisible;
use crate::solution::Solution;

pub struct Day11 { file_name: String }

#[derive(Debug)]
enum Operation {
    Square,
    Multiply(i32),
    Add(i32),
}

#[derive(Debug)]
enum Test {
    Divisible(i32),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: Test,
    on_true: usize,
    on_false: usize,
}

impl Monkey {
    // region parse helpers
    fn new(input: &str) -> Result<Self, String> {
        let mut lines = input.lines().into_iter();
        lines.next(); // dump "Monkey 0:"-line

        let monkey = Monkey {
            items: Monkey::parse_items(lines.next().ok_or_else(|| "Failed to read items")?)?,
            operation: Monkey::parse_operation(lines.next().ok_or_else(|| "Failed to read operation")?)?,
            test: Monkey::parse_test(lines.next().ok_or_else(|| "Failed to read test")?)?,
            on_true: Monkey::parse_if_true(lines.next().ok_or_else(|| "Failed to read if true")?)?,
            on_false: Monkey::parse_if_false(lines.next().ok_or_else(|| "Failed to read if false")?)?,
        };
        Ok(monkey)
    }

    fn parse_items(input: &str) -> Result<Vec<i32>, String> {
        input[18..].split(", ")
            .map(|it| {
                it.parse::<i32>().or_else(|e| Err(format!("Failed to parse item {it}: {e}")))
            }).collect::<Result<Vec<i32>, String>>()
    }

    fn parse_operation(input: &str) -> Result<Operation, String> {
        let input = &input[23..];
        if input == "* old" {
            Ok(Square)
        } else if input.starts_with("*") {
            let n = input[2..].parse::<i32>()
                .or_else(|e| Err(format!("Failed to parse operation: {input} ({e})")))?;
            Ok(Multiply(n))
        } else if input.starts_with("+") {
            let n = input[2..].parse::<i32>()
                .or_else(|e| Err(format!("Failed to parse operation: {input} ({e})")))?;
            Ok(Add(n))
        } else {
            Err(format!("Could not parse operation: {input}"))
        }
    }

    fn parse_test(input: &str) -> Result<Test, String> {
        let input = &input[8..];
        if input.starts_with("divisible by") {
            let n = input[13..].parse::<i32>()
                .or_else(|e| Err(format!("Failed to parse on false: {e}")))?;
            Ok(Divisible(n))
        } else {
            Err(format!("Failed to parse test {input}"))
        }
    }

    fn parse_if_true(input: &str) -> Result<usize, String> {
        input[29..].parse::<usize>()
            .or_else(|e| Err(format!("Failed to parse if true: {e}")))
    }

    fn parse_if_false(input: &str) -> Result<usize, String> {
        input[30..].parse::<usize>()
            .or_else(|e| Err(format!("Failed to parse if false: {e}")))
    }
    // endregion parse helpers
}

impl Day11 {
    fn parse(&self) -> Result<Vec<Monkey>, String> {
        self.read_file_as_string()
            ?.split("\n\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|raw| Monkey::new(raw))
            .collect::<Result<Vec<Monkey>, String>>()
    }
}

impl Solution for Day11 {
    fn new(file_name: String) -> Self { Day11 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }

    fn part1(&self) -> Result<String, String> {
        let mut monkeys = self.parse()?;
        monkeys.iter().for_each(|x| println!("{:?}", x));
        todo!()
    }
}
