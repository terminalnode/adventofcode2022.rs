use crate::solution::Solution;

pub struct Day14 { file_name: String }

impl Day14 {
}

impl Solution for Day14 {
    fn new(file_name: String) -> Self { Day14 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
