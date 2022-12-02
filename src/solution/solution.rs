use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub trait Solution {
    fn new(file_name: String) -> Self;
    fn get_file_name(&self) -> String;

    fn run(&self, part: String) -> Result<String, String> {
        match part.as_str() {
            "1" => self.part1(),
            "2" => self.part2(),
            "all" => Err("I have not yet been arsed to implement 'all'.".to_string()),
            _ => return Err(format!("There is no part {}", part)),
        }
    }

    fn part1(&self) -> Result<String, String> {
        Err("Not implemented yet!".to_string())
    }

    fn part2(&self) -> Result<String, String> {
        Err("Not implemented yet!".to_string())
    }

    fn read_file(&self) -> Result<BufReader<File>, Error> {
        match File::open(self.get_file_name()) {
            Ok(f) => Ok(BufReader::new(f)),
            Err(e) => Err(e),
        }
    }

    fn read_file_as_lines(&self) -> Result<Vec<String>, Error> {
        self.read_file()?.lines().collect()
    }
}
