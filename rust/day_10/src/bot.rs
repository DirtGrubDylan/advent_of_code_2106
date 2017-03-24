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
    pub fn new(id: i32) -> Bot {
        Bot {
            id: id,
            low_value: None,
            high_value: None,
            bot_instructions: VecDeque::new(),
        }
    }

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

        if self.can_execute() {
            self.execute_next_instruction();
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.bot_instructions.push_back(instruction)
    }

    pub fn can_execute(&self) -> bool {
        self.low_value.is_some() && self.high_value.is_some()
    }

    pub fn execute_next_instruction(&mut self) {
        if let Some(instruction) = self.bot_instructions.pop_front() {
            match instruction {
                Instruction::BotInstruction(_,
                                            ref low_dest,
                                            ref low_id,
                                            ref high_dest,
                                            ref high_id) => {
                                            }
                Instruction::ValueInstruction(..) => {
                    panic!("Bot is holding a value instruction: {:?}!", instruction)
                }
            }
        }
    }

    fn give_high_value_to_bot(&mut self, other_bot: &mut Bot) {
        other_bot.add_value(self.high_value.unwrap());
        self.high_value = None;
    }

    fn give_low_value_to_bot(&mut self, other_bot: &mut Bot) {
        other_bot.add_value(self.low_value.unwrap());
        self.low_value = None;
    }

    fn give_high_value_to_output(&mut self, output: &mut Vec<i32>) {
        output.push(self.high_value.unwrap());
        self.high_value = None;
    }

    fn give_low_value_to_output(&mut self, output: &mut Vec<i32>) {
        output.push(self.low_value.unwrap());
        self.low_value = None;
    }
}