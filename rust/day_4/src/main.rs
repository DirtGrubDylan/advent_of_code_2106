extern crate day_4;

use std::env;
use day_4::RoomKey;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let data: Vec<RoomKey> = day_4::load_room_keys(arguments.get(1).unwrap()).unwrap();

    println!("Sum of Real Room Sector IDs: {}",
             day_4::sum_real_roomkey_sector_ids(&data));

    println!("Room Sector ID Containing North Pole Objects: {}",
             data.iter()
                 .find(|roomkey| roomkey.decrypted_name().to_lowercase().contains("north"))
                 .unwrap()
                 .sector_id);
}