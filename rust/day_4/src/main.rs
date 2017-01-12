extern crate day_4;

use std::io;
use std::env;
use std::fs::File;
use day_4::RoomKey;
use std::io::BufRead;

pub fn load_room_keys(data_file_path: &String) -> Result<Vec<RoomKey>, String> {
    let data_file: File = File::open(data_file_path).map_err(|error| error.to_string())?;
    let mut data: Vec<RoomKey> = Vec::new();

    for line in io::BufReader::new(data_file).lines() {
        let roomkey: String = line.map_err(|error| error.to_string())?;
        let roomkey_data: Vec<&str> = roomkey.rsplitn(4, |c| c == '-' || c == '[' || c == ']')
            .filter(|&s| s != "")
            .collect();

        data.push(RoomKey {
            encrypted_name: roomkey_data[2].to_string(),
            sector_id: roomkey_data[1].parse::<u32>().map_err(|error| error.to_string())?,
            check_sum: roomkey_data[0].to_string(),
        });
    }

    Ok(data)
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let data: Vec<RoomKey> = load_room_keys(arguments.get(1).unwrap()).unwrap();

    println!("Sum of Real Room Sector IDs: {}", day_4::sum_real_roomkey_sector_ids(&data))
}

#[cfg(test)]
mod test {
    use super::*;
    use day_4::RoomKey;

    #[test]
    fn test_load_room_keys() {
        let test_data_file_path: String =
            "/home/dhicks/Documents/advent_of_code_2106/python/tests/test_data/test_day4_data.txt"
                .to_string();
        let test_roomkeys: Vec<RoomKey> = vec![RoomKey {
                                                   encrypted_name: "aaaaa-bbb-z-y-x".to_string(),
                                                   sector_id: 123,
                                                   check_sum: "abxyz".to_string(),
                                               },
                                               RoomKey {
                                                   encrypted_name: "a-b-c-d-e-f-g-h".to_string(),
                                                   sector_id: 987,
                                                   check_sum: "abcde".to_string(),
                                               },
                                               RoomKey {
                                                   encrypted_name: "not-a-real-room".to_string(),
                                                   sector_id: 404,
                                                   check_sum: "oarel".to_string(),
                                               },
                                               RoomKey {
                                                   encrypted_name: "totally-real-room".to_string(),
                                                   sector_id: 200,
                                                   check_sum: "decoy".to_string(),
                                               },
                                               RoomKey {
                                                   encrypted_name: "qzmt-zixmtkozy-ivhz"
                                                       .to_string(),
                                                   sector_id: 343,
                                                   check_sum: "zimtb".to_string(),
                                               }];

        assert_eq!(test_roomkeys, load_room_keys(&test_data_file_path).unwrap());
    }
}