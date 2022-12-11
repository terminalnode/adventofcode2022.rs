use crate::solution::Solution;

pub struct Day15 { file_name: String }

impl Day15 {
}

impl Solution for Day15 {
    fn new(file_name: String) -> Self { Day15 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
