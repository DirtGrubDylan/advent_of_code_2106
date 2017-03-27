use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    BotInstruction(i32, String, i32, String, i32),
    ValueInstruction(i32, i32),
}

pub fn instruction_from(data_line: &str) -> Result<Instruction, String> {
    if data_line.starts_with("value") {
        return value_instruction_from(data_line);
    } else if data_line.starts_with("bot") {
        return bot_instruction_from(data_line);
    }

    Err("Not an Instruction.".to_string())
}

fn bot_instruction_from(data_line: &str) -> Result<Instruction, String> {
    let bot_instruction_regex: Regex = Regex::new(r".{4}(\d).{14}(\w+)\s(\d).{13}(\w+)\s(\d)")
        .unwrap();

    match bot_instruction_regex.captures(data_line) {
        Some(captures) => {
            Ok(Instruction::BotInstruction((&captures[1])
                                               .parse::<i32>()
                                               .expect("Not an Instruction."),
                                           (&captures[2]).to_string(),
                                           (&captures[3])
                                               .parse::<i32>()
                                               .expect("Not an Instruction."),
                                           (&captures[4]).to_string(),
                                           (&captures[5])
                                               .parse::<i32>()
                                               .expect("Not an Instruction.")))
        }
        None => Err("Not an Instruction.".to_string()),
    }
}

fn value_instruction_from(data_line: &str) -> Result<Instruction, String> {
    let value_instruction_regex: Regex = Regex::new(r".{6}(\d).*(\d)").unwrap();

    match value_instruction_regex.captures(data_line) {
        Some(captures) => {
            Ok(Instruction::ValueInstruction((&captures[1])
                                                 .parse::<i32>()
                                                 .expect("Not an Instruction."),
                                             (&captures[2])
                                                 .parse::<i32>()
                                                 .expect("Not an Instruction.")))
        }
        None => Err("Not an Instruction.".to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_from() {
        let instructions = vec!["value 5 goes to bot 2",
                                "bot 1 gives low to output 1 and high to bot 0"];

        assert_eq!(instruction_from(instructions[0]).unwrap(),
                   Instruction::ValueInstruction(5, 2));
        assert_eq!(instruction_from(instructions[1]).unwrap(),
                   Instruction::BotInstruction(1, "output".to_string(), 1, "bot".to_string(), 0));
    }

    #[test]
    fn test_bot_instruction_from() {
        let instruction = "bot 1 gives low to output 1 and high to bot 0";

        assert_eq!(bot_instruction_from(instruction).unwrap(),
                   Instruction::BotInstruction(1, "output".to_string(), 1, "bot".to_string(), 0));
    }

    #[test]
    fn test_value_instruction_from() {
        let instruction = "value 5 goes to bot 2";

        assert_eq!(value_instruction_from(instruction).unwrap(),
                   Instruction::ValueInstruction(5, 2));
    }
}