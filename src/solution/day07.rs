use crate::solution::Solution;

pub struct Day07 { file_name: String }

impl Day07 {
}

impl Solution for Day07 {
    fn new(file_name: String) -> Self { Day07 { file_name } }

    fn get_file_name(&self) -> String { return self.file_name.clone() }
}
