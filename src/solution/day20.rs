use crate::solution::Solution;

pub struct Day20 { file_name: String }

impl Day20 {
}

impl Solution for Day20 {
    fn new(file_name: String) -> Self { Day20 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
