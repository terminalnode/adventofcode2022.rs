use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use itertools::Itertools;
use regex::Regex;

use crate::solution::Solution;

pub struct Day05 { file_name: String }

struct Move {
    number: i32,
    start: i32,
    stop: i32,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "move {} from {} to {}", self.number, self.start, self.stop)
    }
}

impl Day05 {
    fn parse(&self) -> Result<(HashMap<i32, Vec<char>>, Vec<Move>), String> {
        let split = self.read_file_as_string()?
            .split("\n\n").map(|s| s.to_string())
            .collect::<Vec<String>>();

        let columns = self.parse_columns(&split[0])?;
        let moves = self.parse_moves(&split[1])?;

        Ok((columns, moves))
    }

    fn parse_columns(&self, data: &str) -> Result<HashMap<i32, Vec<char>>, String> {
        let mut capacity = 0;
        let mut columns = 0;
        for c in data.chars() {
            if c.is_alphabetic() { capacity += 1 }
            else if c.is_digit(10) { columns += 1 }
        }
        let mut map: HashMap<i32, Vec<char>> = (0..columns)
            .map(|i| (i, Vec::<char>::with_capacity(capacity)))
            .collect();

        for line in data.lines().rev().skip(1) {
            let mut itt = line.chars().into_iter();
            itt.next();
            for col in 0..columns {
                itt.next().and_then(|c| {
                    if !c.is_whitespace() {
                        map.get_mut(&col)
                            .and_then(|it| Some(it.push(c)));
                    }

                    // Skip ahead to next char
                    itt.next(); itt.next(); itt.next()
                });
            }
        }

        Ok(map)
    }

    fn parse_moves(&self, data: &str) -> Result<Vec<Move>, String> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        data.lines().map(|line| {
            let c = re.captures(line).unwrap();
            let number = *&c[1].parse::<i32>()
                .or(Err(format!("Failed to parse number from '{}'", line)))?;
            let start = *&c[2].parse::<i32>()
                .or(Err(format!("Failed to parse start from '{}'", line)))?;
            let stop = *&c[3].parse::<i32>()
                .or(Err(format!("Failed to parse stop from '{}'", line)))?;

            Ok(Move { number, start, stop })
        }).collect::<Result<Vec<Move>, String>>()
    }

    fn map_to_result(&self, map: HashMap<i32, Vec<char>>) -> Result<String, String> {
        match map.keys()
            .sorted_by(|a, b| a.cmp(b))
            .map(|it| {
                let vec = map.get(it)
                    .ok_or(format!("Failed to get column {}", it))?;
                let last = vec.last()
                    .ok_or(format!("Failed to get last char in column {}", it))?;
                Ok(*last)
            }).collect::<Result<Vec<char>, String>>() {
            Ok(s) => Ok(s.iter().join("")),
            Err(e) => Err(e),
        }
    }
}

impl Solution for Day05 {
    fn new(file_name: String) -> Self { Day05 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }

    fn part1(&self) -> Result<String, String> {
        let (mut map, moves) = self.parse()?;

        for mv in moves {
            let mut start = map.get(&(mv.start - 1))
                .ok_or(format!("Failed to get start column: {}", mv.start - 1))
                ?.clone();
            let mut stop = map.get(&(mv.stop - 1))
                .ok_or(format!("Failed to get stop column: {}", mv.stop - 1))
                ?.clone();

            for _ in 0..mv.number {
                let pop = start.pop()
                    .ok_or(format!("Failed to pop from column {}: {:?}", mv.start - 1, start))?;
                stop.push(pop);
            }

            map.insert(mv.start - 1, start);
            map.insert(mv.stop - 1, stop);
        }

        self.map_to_result(map)
    }

    fn part2(&self) -> Result<String, String> {
        let (mut map, moves) = self.parse()?;

        for mv in moves {
            let mut start = map.get(&(mv.start - 1))
                .ok_or(format!("Failed to get start column: {}", mv.start - 1))
                ?.clone();
            let mut stop = map.get(&(mv.stop - 1))
                .ok_or(format!("Failed to get stop column: {}", mv.stop - 1))
                ?.clone();

            let mut to_move: Vec<char> = Vec::with_capacity(mv.number as usize);
            for _ in 0..mv.number {
                let pop = start.pop()
                    .ok_or(format!("Failed to pop from column {}: {:?}", mv.start - 1, start))?;
                to_move.push(pop);
            }
            to_move.reverse();
            for it in to_move {
                stop.push(it);
            }

            map.insert(mv.start - 1, start);
            map.insert(mv.stop - 1, stop);
        }

        self.map_to_result(map)
    }
}
