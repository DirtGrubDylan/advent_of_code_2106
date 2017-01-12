extern crate day_4;

use std::io;
use std::env;
use std::fs::File;
use day_4::RoomKey;
use std::io::BufRead;

fn load_room_keys(data_file_path: &String) -> Result<Vec<RoomKey>, String> {
    let data_file: File = File::open(data_file_path).map_err(|error| error.to_string())?;
    let mut data: Vec<RoomKey> = Vec::new();

    for line in io::BufReader::new(data_file).lines() {
        let roomkey: String = line.map_err(|error| error.to_string())?;
        let tester: Vec<&str> = roomkey.rsplitn(2, '-').collect();

        // println!("RoomKey: {}", roomkey);
        // println!("Encrypted Name: {:?}", roomkey.rsplitn(2, '-').collect::<Vec<&str>>());
        // println!("Sector ID: {}", roomkey);
        // println!("Check Sum: {}", roomkey);

        // data.push(RoomKey { encrypter_name: roomkey, sector_id: 0, ce });
        println!(
            "RoomKey {:?}",
            RoomKey {
                encrypted_name: tester[1].to_string(), sector_id: 0,
                check_sum: "This".to_string() });
    }

    Ok(data)
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let data: Vec<RoomKey> = load_room_keys(arguments.get(1).unwrap()).unwrap();
}
