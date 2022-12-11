use crate::solution::Solution;

pub struct Day21 { file_name: String }

impl Day21 {
}

impl Solution for Day21 {
    fn new(file_name: String) -> Self { Day21 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
