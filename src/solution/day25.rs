use crate::solution::Solution;

pub struct Day25 { file_name: String }

impl Day25 {
}

impl Solution for Day25 {
    fn new(file_name: String) -> Self { Day25 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
