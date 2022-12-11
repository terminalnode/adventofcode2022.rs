use std::ops::Not;
use ansi_term::Colour::{Red, Green};
use itertools::Itertools;
use crate::solution::Solution;

pub struct Day08 { file_name: String }

type Matrix<T> = Vec<Vec<T>>;

impl Day08 {
    fn parse(&self) -> Result<Matrix<u32>, String> {
        self.read_file_as_lines()?.iter()
            .map(|s| {
                s.chars().into_iter().map(|c| {
                    match c.to_digit(10) {
                        None => Err(format!("Failed to parse digit '{c}'")),
                        Some(n) => Ok(n),
                    }
                }).collect::<Result<Vec<u32>, String>>()
            }).collect::<Result<Matrix<u32>, String>>()
    }
}

fn green_bold(n: &u32) -> String {
    Green.bold().paint(format!("{n}")).to_string()
}

fn red_bold(n: &u32) -> String {
    Red.bold().paint(format!("{n}")).to_string()
}

impl Solution for Day08 {
    fn new(file_name: String) -> Self { Day08 { file_name } }
    fn get_file_name(&self) -> String { return self.file_name.clone() }

    fn part1(&self) -> Result<String, String> {
        // Note: X/Y axis are reversed. matrix[y_pos][x_pos] is where y = y_pos and x = x_pos
        let matrix = self.parse()?;
        let y_len = matrix.len();
        let x_len = matrix[0].len();
        let mut visible = 0;

        let mut result = matrix.iter().enumerate().map(|(i1, row)| {
            row.iter().enumerate().map(|(i2, n)| {
                if i1 == 0 || i1 == y_len-1 {
                    visible += 1;
                    green_bold(n)
                } else if i2 == 0 || i2 == x_len-1 {
                    visible += 1;
                    green_bold(n)
                } else {
                    format!("{n}")
                }
            }).collect()
        }).collect::<Matrix<String>>();

        for y in 1..y_len-1 {
            for x in 1..x_len-1 {
                let this = matrix[y][x];

                let direction1 = (0..x) .any(|xt| {
                    let that = matrix[y][xt];
                    this <= that
                }).not();

                let direction2 = (x+1..x_len).any(|xt| {
                    let that = matrix[y][xt];
                    this <= that
                }).not();

                let direction3 = (0..y).any(|yt| {
                    let that = matrix[yt][x];
                    this <= that
                }).not();

                let direction4 = (y+1..y_len).any(|yt| {
                    let that = matrix[yt][x];
                    this <= that
                }).not();

                // Update results for this point
                let is_ok = direction1 || direction2 || direction3 || direction4;
                let entry = if is_ok {
                    visible += 1;
                    green_bold(&this)
                } else {
                    red_bold(&this)
                };
                result[y][x] = entry;
            }
        }

        let forest = result.iter().map(|it| it.join("")).join("\n");
        Ok(format!("\n{forest}\n{visible} trees visible"))
    }

    fn part2(&self) -> Result<String, String> {
        let matrix = self.parse()?;
        let y_len = matrix.len();
        let x_len = matrix[0].len();

        let mut result = matrix.iter().map(|row| {
            row.iter().map(|n| (n.clone(), 0)).collect::<Vec<(u32, u32)>>()
        }).collect::<Matrix<(u32, u32)>>();

        for y in 1..y_len-1 {
            for x in 1..x_len-1 {
                let this = matrix[y][x];

                let mut left = 0;
                for tx in (0..x).rev() {
                    left += 1;
                    let that = matrix[y][tx];
                    if that >= this { break }
                }

                let mut right = 0;
                for tx in x+1..x_len {
                    right += 1;
                    let that = matrix[y][tx];
                    if that >= this { break }
                }

                let mut up = 0;
                for ty in (0..y).rev() {
                    up += 1;
                    let that = matrix[ty][x];
                    if that >= this { break }
                }

                let mut down = 0;
                for ty in y+1..y_len {
                    down += 1;
                    let that = matrix[ty][x];
                    if that >= this { break }
                }

                result[&y+0][&x+0] = (this, left * right * up * down);
            }
        }

        match result.iter().flat_map(|it| {
            it.iter().map(|(_, y)| y+0).collect::<Vec<u32>>()
        }).collect::<Vec<u32>>().iter().max() {
            None => Err("Failed to get result".to_string()),
            Some(n) => Ok(format!("Max scenic score: {n}")),
        }
    }
}
