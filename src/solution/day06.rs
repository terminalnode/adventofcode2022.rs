use crate::solution::Solution;

pub struct Day06 { file_name: String }

impl Day06 {
}

impl Solution for Day06 {
    fn new(file_name: String) -> Self { Day06 { file_name } }

    fn get_file_name(&self) -> String { return self.file_name.clone() }
}
