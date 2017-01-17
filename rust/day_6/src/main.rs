extern crate day_6;

use std::env;

fn main() {
    let arguments = env::args().collect::<Vec<String>>();

    let data = day_6::load_messages_from_file(&arguments[1]).unwrap();

    println!("First Answer: {}", day_6::message_from_most_used(&data));
    println!("Second Answer: {}", day_6::message_from_least_used(&data));
}
