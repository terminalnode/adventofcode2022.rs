use std::collections::HashSet;
use crate::solution::Solution;

pub struct Day06 { file_name: String }

impl Day06 {
}

impl Solution for Day06 {
    fn new(file_name: String) -> Self { Day06 { file_name } }

    fn get_file_name(&self) -> String { return self.file_name.clone() }

    fn part1(&self) -> Result<String, String> {
        let raw = self.read_file_as_string()?;
        let s: Vec<char> = raw.chars().collect();

        match (0..s.len() - 2)
            .find(|idx| {
                let i = *idx;
                let mut set: HashSet<char> = HashSet::new();
                set.insert(s[i]);
                set.insert(s[i+1]);
                set.insert(s[i+2]);
                set.insert(s[i+3]);
                println!("{:?} = {:?}", set, set.len() == 4);
                set.len() == 4
            }) {
            None => Err("Failed to find start of signal".to_string()),
            Some(n) => Ok(format!("{}", n+4))
        }
    }

    fn part2(&self) -> Result<String, String> {
        let raw = self.read_file_as_string()?;
        let s: Vec<char> = raw.chars().collect();

        match (0..s.len() - 12)
            .find(|idx| {
                let mut set: HashSet<char> = HashSet::new();
                (0..14).for_each(|it| { set.insert(s[it+idx]); });
                println!("{:?} = {:?}", set, set.len() == 4);
                set.len() == 14
            }) {
            None => Err("Failed to find start of signal".to_string()),
            Some(n) => Ok(format!("{}", n+14))
        }
    }
}
