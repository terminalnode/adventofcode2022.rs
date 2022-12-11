use crate::solution::Solution;

pub struct Day16 { file_name: String }

impl Day16 {
}

impl Solution for Day16 {
    fn new(file_name: String) -> Self { Day16 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
