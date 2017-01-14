extern crate crypto;

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
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_id_with_iteration() {
        let test_door: Door = Door("abc".to_string());

        assert!(test_door.hash_id_with_iteration(5017308).starts_with("000008f82"));
        assert!(test_door.hash_id_with_iteration(3231929).starts_with("000001"));
        assert!(test_door.hash_id_with_iteration(5278568).starts_with("00000f"));
    }


    #[test]
    fn test_l_to_r_password() {
        let test_door: Door = Door("abc".to_string());

        assert_eq!(test_door.l_to_r_password(), "18f47a30");
    }
}