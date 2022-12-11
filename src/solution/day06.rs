use std::collections::HashSet;

use crate::solution::Solution;

pub struct Day06 { file_name: String }

impl Day06 {
    fn solve(&self, uniques: usize) -> Result<String, String> {
        let raw = self.read_file_as_string()?;
        let s: Vec<char> = raw.chars().collect();

        match (0..s.len() - (uniques - 2))
            .find(|idx| {
                let mut set: HashSet<char> = HashSet::new();
                (0..uniques).for_each(|it| { set.insert(s[it+idx]); });
                set.len() == uniques
            }) {
            None => Err("Failed to find start of signal".to_string()),
            Some(n) => Ok(format!("{}", n+uniques))
        }
    }
}

impl Solution for Day06 {
    fn new(file_name: String) -> Self { Day06 { file_name } }
    fn get_file_name(&self) -> String { return self.file_name.clone() }
    fn part1(&self) -> Result<String, String> { self.solve(4) }
    fn part2(&self) -> Result<String, String> { self.solve(14) }
}
