extern crate day_10;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let instructions = day_10::instructions_from(&args[1]).unwrap();

    for (index, instruction) in instructions.iter().enumerate() {
        println!("Instructions #{}: {:?}", index, instruction);
    }
}
