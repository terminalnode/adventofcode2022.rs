use crate::solution::Solution;

pub struct Day07 { file_name: String }

#[derive(Debug)]
struct Directory<'a> {
    name: &'a str,
    directories: Vec<Directory<'a>>,
    files_size: i32,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Self {
        Directory {
            name,
            directories: Default::default(),
            files_size: 0,
        }
    }

    fn add_file(&mut self, size: i32) {
        self.files_size += size;
    }

    fn get_total(&self) -> i32 {
        let sub_size: i32 = self.directories.iter().map(Directory::get_total).sum();
        sub_size + self.files_size
    }
}

impl Day07 {
    fn parse(&self) -> Directory {
        Directory::new("/")
    }
}

impl Solution for Day07 {
    fn new(file_name: String) -> Self { Day07 { file_name } }

    fn get_file_name(&self) -> String { return self.file_name.clone() }

    fn part1(&self) -> Result<String, String> {
        let cwd = self.parse();
        println!("{:?}", cwd);
        todo!()
    }
}
