use crate::solution::Solution;

pub struct Day18 { file_name: String }

impl Day18 {
}

impl Solution for Day18 {
    fn new(file_name: String) -> Self { Day18 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
