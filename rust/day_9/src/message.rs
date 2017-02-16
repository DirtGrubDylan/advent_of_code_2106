use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Message {
    pub data: String,
}

impl Message {
    pub fn new(data: &str) -> Message {
        Message { data: data.to_string() }
    }

    pub fn decompress_v1(&self) -> u32 {
        let marker_finder: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

        decompressed_string_length_v1(&self.data, marker_finder)
    }

    pub fn decompress_v2(&self) -> u32 {
        let marker_finder: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

        decompress_string_v2(&self.data, marker_finder)
    }
}


fn decompressed_string_length_v1(message: &str, marker_regex: Regex) -> u32 {
    let mut decompressed_length: u32 = 0;
    let mut str_index: usize = 0;

    while let Some(matched) = marker_regex.find(&message[str_index..]) {
        let captured = marker_regex.captures(&message[str_index..]).unwrap();

        decompressed_length += matched.start() as u32;

        let number_of_chars: usize = (&captured[1]).parse().unwrap();
        let number_of_repeats: usize = (&captured[2]).parse().unwrap();

        str_index += matched.end() + number_of_chars;

        decompressed_length += (number_of_repeats * number_of_chars) as u32;
    }

    if str_index < message.len() {
        decompressed_length += (message.len() - str_index) as u32;
    }

    decompressed_length
}

fn decompress_string_v2(message: &str, marker_regex: Regex) -> u32 {
    let mut decompressed_length: u32 = 0;
    let mut str_index: usize = 0;

    decompressed_length
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_messages_from_file() {
        let test_path = "/home/dhicks/Documents/advent_of_code_2106/python/tests/test_data/test_day9_data.\
                         txt";

        let test_data = vec![Message::new("ADVENT"),
                             Message::new("A(1x5)BC"),
                             Message::new("(3x3)XYZ"),
                             Message::new("A(2x2)BCD(2x2)EFG"),
                             Message::new("(6x1)(1x3)A"),
                             Message::new("X(8x2)(3x3)ABCY"),
                             Message::new("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
                             Message::new("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")];
    }
}