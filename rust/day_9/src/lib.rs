extern crate regex;

mod message;

use std::io;
use std::fs::File;
use std::io::BufRead;

pub use message::Message;

pub fn load_messages_from_file(file_name: &str) -> io::Result<Vec<Message>> {
    let data: File = File::open(file_name)?;
    let mut message_data: Vec<Message> = Vec::new();

    for data_line in std::io::BufReader::new(data).lines() {
        message_data.push(Message::new(&(data_line?)));
    }

    Ok(message_data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_messages_from_file() {
        let test_path = r"C:\Users\dylan\Documents\advent_of_code_2106\python\tests\test_data\test_day9_data.txt";

        let test_data = vec![Message::new("ADVENT"),
                             Message::new("A(1x5)BC"),
                             Message::new("(3x3)XYZ"),
                             Message::new("A(2x2)BCD(2x2)EFG"),
                             Message::new("(6x1)(1x3)A"),
                             Message::new("X(8x2)(3x3)ABCY"),
                             Message::new("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
                             Message::new("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")];

        assert_eq!(load_messages_from_file(test_path).unwrap(), test_data);
    }
}