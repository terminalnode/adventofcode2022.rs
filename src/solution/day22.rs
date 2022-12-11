use crate::solution::Solution;

pub struct Day22 { file_name: String }

impl Day22 {
}

impl Solution for Day22 {
    fn new(file_name: String) -> Self { Day22 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
