use crate::solution::Solution;

pub struct Day09 { file_name: String }

impl Day09 {
}

impl Solution for Day09 {
    fn new(file_name: String) -> Self { Day09 { file_name } }
    fn get_file_name(&self) -> String {
        self.file_name.clone()
    }
}
