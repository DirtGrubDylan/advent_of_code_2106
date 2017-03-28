extern crate day_10;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let instructions = day_10::instructions_from(&args[1]).unwrap();

    let mut bot_manager = day_10::BotManager::new((&args[2]).parse().unwrap(),
                                                  (&args[3]).parse().unwrap());

    bot_manager.run_bots_with(&instructions);

    println!("Answer 1: {}", bot_manager.interesting_bot_id);

    let second_answer = bot_manager.output_bins.get(&0).unwrap()[0] *
                        bot_manager.output_bins.get(&1).unwrap()[0] *
                        bot_manager.output_bins.get(&2).unwrap()[0];

    println!("Answer 2: {}", second_answer);
}
