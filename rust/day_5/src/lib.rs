extern crate crypto;

use crypto::md5;
use crypto::digest::Digest;

#[derive(Debug)]
pub struct Door(pub String);

impl Door {
    pub fn l_to_r_password(&self) -> String {
        let mut tester: md5::Md5 = md5::Md5::new();

        tester.input_str(&self.0);

        tester.result_str()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_l_to_r_password() {
        let mut test_door: Door = Door("abc5017308".to_string());

        assert!(test_door.l_to_r_password().starts_with("000008f82"));

        test_door = Door("abc3231929".to_string());

        assert!(test_door.l_to_r_password().starts_with("000001"));

        test_door = Door("abc5278568".to_string());

        assert!(test_door.l_to_r_password().starts_with("00000f"));
    }
}