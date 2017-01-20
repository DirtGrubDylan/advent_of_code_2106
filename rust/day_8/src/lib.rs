mod screen;
mod instructions;

use std::io;
use std::fs::File;
use std::io::BufRead;

pub use screen::Screen;

use instructions::Instruction;


pub fn load_instructions_from(file_path: &str) -> io::Result<Vec<String>> {
    let data_file = File::open(file_path)?;
    let mut data = Vec::new();

    for line in io::BufReader::new(data_file).lines() {
        data.push(line?);
    }

    Ok(data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_instructions_from() {
        let test_instructions = vec!["rect 3x3",
                                     "rotate column x=1 by 1",
                                     "rotate row y=0 by 4",
                                     "rotate column x=1 by 1"];

        assert_eq!(
            load_instructions_from(
                "/home/dhicks/Documents/advent_of_code_2106/python/tests/test_data/\
                test_day8_data.txt").unwrap(),
            test_instructions);
    }
}
