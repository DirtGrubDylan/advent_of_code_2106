extern crate day_2;

use std::io;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use day_2::keypad;

fn main() {
    let mut data_path: PathBuf = env::current_dir().unwrap();
    data_path.push("data");
    data_path.push("input.txt");

    let data_lines: io::Result<io::Lines<io::BufReader<File>>> =
        day_2::read_instructions(&data_path);

    let data_lines: Vec<String> =
        day_2::buffer_lines_to_string_vec(data_lines.unwrap()).unwrap();

    let ebhq_keypad_1: keypad::KeyPad = keypad::KeyPad {
        keys: "123456789".to_owned(),
        width: 3
    };
    let start_position_1: [usize; 2] = [1, 1];

    let ebhq_keypad_2: keypad::KeyPad = keypad::KeyPad {
        keys: "__1___234_56789_ABC___D__".to_owned(),
        width: 5
    };
    let start_position_2: [usize; 2] = [2, 0];

    let answer_1: String = day_2::interpret_instructions(
        &data_lines, &ebhq_keypad_1, start_position_1).unwrap();

    let answer_2: String = day_2::interpret_instructions(
        &data_lines, &ebhq_keypad_2, start_position_2).unwrap();

    println!("Answer 1: {}", answer_1);
    println!("Answer 2: {}", answer_2);
}
