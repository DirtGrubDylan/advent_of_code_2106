mod roomkey;

pub use self::roomkey::RoomKey;

pub fn sum_real_roomkey_sector_ids(roomkeys: &Vec<RoomKey>) -> u32 {
    roomkeys.iter().fold(0, |accumulator, roomkey| {
        if roomkey.is_real() {
            accumulator + roomkey.sector_id
        } else {
            accumulator
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_real_roomkey_sector_ids() {
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

        assert_eq!(sum_real_roomkey_sector_ids(&test_roomkeys), 1514);
    }
}
