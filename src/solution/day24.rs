use crate::solution::Solution;

pub struct Day24 { file_name: String }

impl Day24 {
}

impl Solution for Day24 {
    fn new(file_name: String) -> Self { Day24 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
