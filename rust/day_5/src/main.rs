extern crate day_5;

use day_5::Door;

use std::env;
use std::time::Instant;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let time_now: Instant = Instant::now();
    let locked_door: Door = Door(arguments[1].clone());

    println!("The First Password is: {}", locked_door.l_to_r_password());

    let new_password: String = locked_door.in_place_password(false);

    println!("The Second Password is: {}", new_password);

    println!("Time to decode passwords: {}", time_now.elapsed().as_secs());
}