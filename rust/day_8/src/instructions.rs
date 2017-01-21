#[derive(Debug, PartialEq)]
pub struct Instruction(pub String, pub u32, pub u32);

impl Instruction {
    pub fn new(instruction: &str) -> Instruction {
        let split_string: Vec<&str> = instruction.split(|c| c == ' ' || c == 'x' || c == '=')
            .collect();

        let mut first_number: u32 = split_string[split_string.len() - 3].parse().unwrap_or(0);
        let second_number: u32 = split_string[split_string.len() - 1].parse().unwrap_or(0);

        match split_string[0] {
            "rect" => {
                first_number = split_string[split_string.len() - 2].parse().unwrap_or(0);
                Instruction(split_string[0].to_string(), first_number, second_number)
            }
            "rotate" => Instruction(split_string[1].to_string(), first_number, second_number),
            _ => Instruction("".to_string(), 0, 0),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(vec![Instruction::new("rect 3x3"),
                        Instruction::new("rotate column x=1 by 1"),
                        Instruction::new("rotate row y=0 by 4"),
                        Instruction::new("rotate column x=1 by 1")],
                   vec![Instruction("rect".to_string(), 3, 3),
                        Instruction("column".to_string(), 1, 1),
                        Instruction("row".to_string(), 0, 4),
                        Instruction("column".to_string(), 1, 1)]);
    }
}