use crate::solution::Solution;

pub struct Day17 { file_name: String }

impl Day17 {
}

impl Solution for Day17 {
    fn new(file_name: String) -> Self { Day17 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
