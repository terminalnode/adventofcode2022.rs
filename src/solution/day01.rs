use crate::solution::Solution;

pub struct Day01 { file_name: String }

impl Day01 {
    pub fn get_counts(&self) -> Result<Vec<i32>, String> {
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

impl Solution for Day01 {
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
