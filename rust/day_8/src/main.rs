extern crate libday_8;

use std::env;
use libday_8::Screen;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut screen = Screen::new(50, 6);

    let instructions = libday_8::load_instructions_from(&arguments[1]).unwrap();

    for instruction in instructions {
        libday_8::interpret_instruction(instruction, &mut screen);
    }

    println!("First answer: {}", screen.number_of_lit_pixels());
    println!("Second answer: {}", screen);
}
