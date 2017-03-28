use std::collections::HashMap;
use std::collections::VecDeque;

use bot::Bot;
use instructions::Instruction;

#[derive(Debug)]
pub struct BotManager {
    output_bins: HashMap<i32, Vec<i32>>,
    bots: HashMap<i32, Bot>,
    interesting_bot_id: i32,
    interesting_low_value: i32,
    interesting_high_value: i32,
}

impl BotManager {
    fn execute(&mut self, bot_id: i32) {
        if self.bots.get(&bot_id).unwrap().contains(self.interesting_low_value, self.interesting_high_value) {
            self.interesting_bot_id = bot_id;
        }

        if let Some(bot_instruction) = self.bots.get_mut(&bot_id).unwrap().next_instruction() {
            if let Instruction::BotInstruction(_, low_dest, low_id, high_dest, high_id) =
                bot_instruction {
                // Low
                if low_dest == "bot" {
                    {
                        let low_val = self.bots.get_mut(&bot_id).unwrap().get_low_val().unwrap();
                        let low_bot = self.bots.entry(low_id).or_insert(Bot::new(low_id));

                        low_bot.add_value(low_val);
                    }

                    if self.bots.get(&low_id).unwrap().can_execute() {
                        self.execute(low_id);
                    }
                } else {
                    let low_val = self.bots.get_mut(&bot_id).unwrap().get_low_val().unwrap();
                    let low_output = self.output_bins.entry(low_id).or_insert(Vec::new());

                    low_output.push(low_val);
                }

                // High
                if high_dest == "bot" {
                    {
                        let high_val = self.bots.get_mut(&bot_id).unwrap().get_high_val().unwrap();
                        let high_bot = self.bots.entry(high_id).or_insert(Bot::new(high_id));

                        high_bot.add_value(high_val);
                    }

                    if self.bots.get(&high_id).unwrap().can_execute() {
                        self.execute(high_id);
                    }
                } else {
                    let high_val = self.bots.get_mut(&bot_id).unwrap().get_high_val().unwrap();
                    let high_output = self.output_bins.entry(high_id).or_insert(Vec::new());

                    high_output.push(high_val);
                }
            }
        }
    }

    pub fn new(interesting_low_value: i32, interesting_high_value: i32) -> BotManager {
        BotManager {
            output_bins: HashMap::new(),
            bots: HashMap::new(),
            interesting_bot_id: -1,
            interesting_low_value: interesting_low_value,
            interesting_high_value: interesting_high_value,
        }
    }

    pub fn run_bots_with(&mut self, instructions: &VecDeque<Instruction>) {
        for instruction in instructions {
            let current_bot_id = match *instruction {
                Instruction::BotInstruction(id, ..) => {
                    self.bots
                        .entry(id)
                        .or_insert(Bot::new(id))
                        .add_instruction(instruction.clone());

                    id
                }
                Instruction::ValueInstruction(value, id) => {
                    self.bots.entry(id).or_insert(Bot::new(id)).add_value(value);

                    id
                }
            };

            if self.bots.get(&current_bot_id).unwrap().can_execute() {
                self.execute(current_bot_id);
            }
        }
    }
}
