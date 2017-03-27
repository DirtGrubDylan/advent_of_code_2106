use std::collections::HashMap;
use std::collections::VecDeque;

use bot::Bot;
use instructions::Instruction;

#[derive(Debug)]
pub struct BotManager {
    output_bins: HashMap<i32, Vec<i32>>,
    bots: HashMap<i32, Bot>,
}

impl BotManager {
    fn execute(&mut self, bot_id: i32) {
        if let Some(bot_instruction) = self.bots.get(&bot_id).unwrap().next_instruction() {
            // Low
            let low_id = bot_instruction.2;

            if bot_instruction.1 == "bot".to_string() {
                let bot = self.bots.entry(low_id).or_insert(Bot::new(low_id));
            } else {
                let output = self.output_bins.entry(low_id).or_insert(Vec::new());
            }

            // High
            let high_id = bot_instruction.4;

            if bot_instruction.3 == "bot".to_string() {
                let bot = self.bots.entry(high_id).or_insert(Bot::new(high_id));
            } else {
                let output = self.output_bins.entry(high_id).or_insert(Vec::new());
            }
        }
    }

    pub fn new() -> BotManager {
        BotManager {
            output_bins: HashMap::new(),
            bots: HashMap::new(),
        }
    }

    pub fn run_bots_with(&mut self, instructions: &VecDeque<Instruction>) {
        let mut current_bot_id = -1;

        for instruction in instructions {
            match *instruction {
                Instruction::BotInstruction(ref id, ..) => {
                    current_bot_id = *id;
                    self.bots.entry(*id).or_insert(Bot::new(*id))
                             .add_instruction(instruction.clone());
                }
                Instruction::ValueInstruction(ref value, ref id) => {
                    current_bot_id = *id;
                    self.bots.entry(*id).or_insert(Bot::new(*id)).add_value(*value);
                }
            }

            if self.bots.get(&current_bot_id).unwrap().can_execute() {
                self.execute(current_bot_id);
            }
        }
    }
}
