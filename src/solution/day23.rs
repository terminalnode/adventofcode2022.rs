use crate::solution::Solution;

pub struct Day23 { file_name: String }

impl Day23 {
}

impl Solution for Day23 {
    fn new(file_name: String) -> Self { Day23 { file_name } }
    fn get_file_name(&self) -> String { self.file_name.clone() }
}
