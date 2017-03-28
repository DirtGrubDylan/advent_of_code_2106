extern crate regex;

mod bot;
mod bot_manager;
mod instructions;

use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::VecDeque;

use instructions::Instruction;

pub fn instructions_from<P: AsRef<Path>>(path: P) -> io::Result<VecDeque<Instruction>> {
    let file = File::open(path)?;
    let mut data: VecDeque<Instruction> = VecDeque::new();

    for line in BufReader::new(file).lines() {
        data.push_back(instructions::instruction_from(&(line?)).unwrap());
    }

    Ok(data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instructions_from() {
        let test_data_path = "../../python/tests/test_data/test_day10_data.txt";
        let mut test_data: VecDeque<instructions::Instruction> = VecDeque::new();

        test_data.push_back(instructions::Instruction::ValueInstruction(5, 2));
        test_data.push_back(instructions::Instruction::BotInstruction(2,
                                                                      "bot".to_string(),
                                                                      1,
                                                                      "bot".to_string(),
                                                                      0));
        test_data.push_back(instructions::Instruction::ValueInstruction(3, 1));
        test_data.push_back(instructions::Instruction::BotInstruction(1,
                                                                      "output".to_string(),
                                                                      1,
                                                                      "bot".to_string(),
                                                                      0));
        test_data.push_back(instructions::Instruction::BotInstruction(0,
                                                                      "output".to_string(),
                                                                      2,
                                                                      "output".to_string(),
                                                                      0));
        test_data.push_back(instructions::Instruction::ValueInstruction(2, 2));

        assert_eq!(instructions_from(test_data_path).unwrap(), test_data);
    }
}
