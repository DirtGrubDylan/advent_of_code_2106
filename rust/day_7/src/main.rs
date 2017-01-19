extern crate libday_7;

use std::env;

use libday_7::IPv7;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let data: Vec<IPv7> = libday_7::load_ipv7s_from_file(&arguments[1]).unwrap();

    println!("First answer: {}", libday_7::number_of_tls_supported(&data));
}
