use crate::solution::Solution;

pub struct Day19 { file_name: String }

impl Day19 {
}

impl Solution for Day19 {
    fn new(file_name: String) -> Self { Day19 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
