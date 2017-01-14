extern crate crypto;

use std::str::Chars;

use crypto::md5;
use crypto::digest::Digest;

#[derive(Debug)]
pub struct Door(pub String);


impl Door {
    pub fn hash_id_with_iteration(&self, iteration: u32) -> String {
        let mut tester: md5::Md5 = md5::Md5::new();
        let id_with_iteration: String = (self.0).clone() + &iteration.to_string();

        tester.input_str(&id_with_iteration);

        tester.result_str()
    }


    pub fn l_to_r_password(&self) -> String {
        let mut iteration: u32 = 0;
        let mut password: String = String::new();

        while password.len() != 8 {
            let hashed_id: String = self.hash_id_with_iteration(iteration);

            if hashed_id.starts_with("00000") {
                password.push_str(&hashed_id[5..6]);
            }

            iteration += 1;
        }

        password
    }


    pub fn in_place_password(&self, printing: bool) -> String {
        let mut iteration: u32 = 0;
        let mut number_of_assigned_characters: u32 = 0;
        let mut password: Vec<char> = vec!['_', '_', '_', '_', '_', '_', '_', '_'];

        while number_of_assigned_characters < 8 {
            let hashed_id: String = self.hash_id_with_iteration(iteration);


            if hashed_id.starts_with("00000") {
                let mut hashed_id: Chars = hashed_id.chars();

                let index: u32 = hashed_id.nth(5).unwrap_or('_').to_digit(10).unwrap_or(8);

                if let Some(element) = password.get_mut(index as usize) {
                    if *element == '_' {
                        *element = hashed_id.nth(0).unwrap_or('_');
                        number_of_assigned_characters += 1;
                    }
                }
            }

            if printing {
                print!("Password: {:?}\r", password);
            }

            iteration += 1;
        }

        if printing {
            println!("");
        }

        password.into_iter().collect::<String>()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_id_with_iteration() {
        let test_door: Door = Door("abc".to_string());

        assert!(test_door.hash_id_with_iteration(5017308).starts_with("000008f82"));
        assert!(test_door.hash_id_with_iteration(3231929).starts_with("0000015"));
        assert!(test_door.hash_id_with_iteration(5278568).starts_with("00000f"));
        assert!(test_door.hash_id_with_iteration(5357525).starts_with("000004e"));
    }


    #[test]
    fn test_l_to_r_password() {
        let test_door: Door = Door("abc".to_string());

        assert_eq!(test_door.l_to_r_password(), "18f47a30");
    }


    #[test]
    fn test_in_place_password() {
        let test_door: Door = Door("abc".to_string());

        assert_eq!(test_door.in_place_password(false), "05ace8e3");
    }
}