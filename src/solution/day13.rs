use crate::solution::Solution;

pub struct Day13 { file_name: String }

impl Day13 {
}

impl Solution for Day13 {
    fn new(file_name: String) -> Self { Day13 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
