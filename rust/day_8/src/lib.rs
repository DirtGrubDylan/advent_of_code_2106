mod screen;
mod instructions;

use std::io;
use std::fs::File;
use std::io::BufRead;

pub use screen::Screen;
pub use instructions::Instruction;

pub fn interpret_instruction(instruction: Instruction, screen: &mut Screen) -> () {
    match &instruction.0 as &str {
        "rect" => {screen.fill_rectangle(instruction.1 as usize, instruction.2 as usize)},
        "column" => {screen.rotate_column(instruction.1 as usize, instruction.2)},
        "row" => {screen.rotate_row(instruction.1 as usize, instruction.2)},
        _ => {},
    }
}

pub fn load_instructions_from(file_path: &str) -> io::Result<Vec<Instruction>> {
    let data_file = File::open(file_path)?;
    let mut data = Vec::new();

    for line in io::BufReader::new(data_file).lines() {
        data.push(Instruction::new(&(line?)));
    }

    Ok(data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_instructions_from() {
        let test_instructions = vec![Instruction("rect".to_string(), 3, 3),
                                     Instruction("column".to_string(), 1, 1),
                                     Instruction("row".to_string(), 0, 4),
                                     Instruction("column".to_string(), 1, 1)];

        assert_eq!(
            load_instructions_from(
                "/home/dhicks/Documents/advent_of_code_2106/python/tests/test_data/\
                test_day8_data.txt").unwrap(),
            test_instructions);
    }
}
