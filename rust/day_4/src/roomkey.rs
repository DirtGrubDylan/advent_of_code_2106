use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct RoomKey {
    pub encrypted_name: String,
    pub sector_id: u32,
    pub check_sum: String,
}

impl RoomKey {
    pub fn is_real(&self) -> bool {
        let mut ordered_char_counter: BTreeMap<char, u32> = BTreeMap::new();

        for character in self.encrypted_name.chars() {
            if character != '-' {
                *ordered_char_counter.entry(character).or_insert(0) += 1;
            }
        }

        let mut ordered_char_counter: Vec<(char, u32)> = ordered_char_counter.into_iter().collect();

        ordered_char_counter.sort_by(|a, b| (&b.1).cmp(&a.1));

        let five_most_used_characters: String = ordered_char_counter[..5]
            .into_iter()
            .fold("".to_string(), |mut accumulator, &(character, _)| {
                accumulator.push(character);
                accumulator
            });

        five_most_used_characters == self.check_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_real() {
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

        let test_real_roomkeys: Vec<bool> =
            test_roomkeys.iter().map(|roomkey| roomkey.is_real()).collect();

        assert_eq!(test_real_roomkeys, vec![true, true, true, false, false]);
    }
}