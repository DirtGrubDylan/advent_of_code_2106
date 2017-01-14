extern crate day_5;

use day_5::Door;

fn main() {
    let locked_door: Door = Door("wtnhxymk".to_string());

    println!("The First Password is: {}", locked_door.l_to_r_password());
}