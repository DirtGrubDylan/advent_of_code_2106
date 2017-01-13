extern crate day_5;

use day_5::Door;

fn main() {
    let tester: Door = Door("abc5017308".to_string());

    println!("Door: {:?}; Hash: {}", tester, tester.l_to_r_password());
}
