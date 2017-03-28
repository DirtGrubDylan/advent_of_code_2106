use std::collections::VecDeque;

use instructions::Instruction;

#[derive(Debug)]
pub struct Bot {
    id: i32,
    low_value: Option<i32>,
    high_value: Option<i32>,
    bot_instructions: VecDeque<Instruction>,
}

impl Bot {
    pub fn add_value(&mut self, value: i32) {
        if let Some(low_val) = self.low_value {
            if low_val <= value {
                self.high_value = Some(value)
            } else {
                self.high_value = self.low_value;
                self.low_value = Some(value)
            }
        } else {
            self.low_value = Some(value);
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::BotInstruction(..) => self.bot_instructions.push_back(instruction),
            Instruction::ValueInstruction(..) => panic!("Cannot use Value Instruction!"),
        }
    }

    pub fn can_execute(&self) -> bool {
        self.low_value.is_some() && self.high_value.is_some()
    }

    pub fn contains(&self, low_val: i32, high_val: i32) -> bool {
        if let Some(low_value) = self.low_value {
            if let Some(high_value) = self.high_value {
                return (low_value == low_val) && (high_value == high_val);
            }
        }

        false
    }

    pub fn get_low_val(&mut self) -> Option<i32> {
        let low_val = self.low_value;

        self.low_value = None;

        low_val
    }

    pub fn get_high_val(&mut self) -> Option<i32> {
        let high_val = self.high_value;

        self.high_value = None;

        high_val
    }

    pub fn new(id: i32) -> Bot {
        Bot {
            id: id,
            low_value: None,
            high_value: None,
            bot_instructions: VecDeque::new(),
        }
    }

    pub fn next_instruction(&mut self) -> Option<Instruction> {
        self.bot_instructions.pop_front()
    }
}