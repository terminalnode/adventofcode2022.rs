use itertools::Itertools;

use crate::solution::Solution;

pub struct Day03 { file_name: String }

fn map_char_to_prio(c: char) -> Result<i32, String> {
    let sub = if c <= 'Z' { 38 } else { 96 };
    let prio = (c as u32 - sub) as i32;
    if prio < 1 || prio > 52 {
        return Err(format!("Failed to convert '{}' to priority", c))
    }
    Ok(prio)
}

impl Day03 {
    fn get_items_as_priorities(&self) -> Result<Vec<Vec<i32>>, String> {
        let lines = self.read_file_as_lines()?;
        lines.iter().map(|it| {
            it.chars()
                .map(map_char_to_prio)
                .collect::<Result<Vec<i32>, String>>()
        }).collect()
    }
}

impl Solution for Day03 {
    fn new(file_name: String) -> Self { Day03 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }

    fn part1(&self) -> Result<String, String> {
        let items_as_priorities = match self.get_items_as_priorities() {
            Ok(ps) => ps,
            Err(e) => return Err(e),
        };

        let result = items_as_priorities.iter()
            .map(|rucksack| {
                let mid = rucksack.len() / 2;
                let first = &rucksack[..mid];
                let second = &rucksack[mid..];

                first.iter()
                    .filter(|it| second.contains(*it))
                    .dedup()
                    .sum::<i32>()
            }).sum::<i32>();

        Ok(format!("Sum of duplicate types: {}", result))
    }

    fn part2(&self) -> Result<String, String> {
        let items_as_priorities = match self.get_items_as_priorities() {
            Ok(ps) => ps,
            Err(e) => return Err(e),
        };

        match items_as_priorities
            .chunks(3).into_iter()
            .map(|chunk| {
                let e1 = &chunk[0];
                let e2 = &chunk[1];
                let e3 = &chunk[2];

                match e1.iter()
                    .find(|it| e2.contains(*it) && e3.contains(*it)) {
                    None => Err("Failed to find matching badge for one of the groups".to_string()),
                    Some(n) => Ok(n.clone()),
                }
            }).sum::<Result<i32, String>>() {
            Ok(n) => Ok(format!("Sum of all badges: {}", n)),
            Err(e) => Err(e),
        }
    }
}
