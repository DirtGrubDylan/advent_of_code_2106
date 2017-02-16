use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Message {
    pub data: String,
}

impl Message {
    pub fn new(data: &str) -> Message {
        Message { data: data.to_string() }
    }

    pub fn decompress_v1(&self) -> u64 {
        let marker_finder: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

        decompressed_string_length_v1(&self.data, marker_finder)
    }

    pub fn decompress_v2(&self) -> u64 {
        let marker_finder: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

        decompress_string_v2(&self.data, &marker_finder)
    }
}


fn decompressed_string_length_v1(message: &str, marker_regex: Regex) -> u64 {
    let mut decompressed_length: u64 = 0;
    let mut str_index: usize = 0;

    while let Some(matched) = marker_regex.find(&message[str_index..]) {
        let captured = marker_regex.captures(&message[str_index..]).unwrap();

        decompressed_length += matched.start() as u64;

        let number_of_chars: usize = (&captured[1]).parse().unwrap();
        let number_of_repeats: usize = (&captured[2]).parse().unwrap();

        str_index += matched.end() + number_of_chars;

        decompressed_length += (number_of_repeats * number_of_chars) as u64;
    }

    if str_index < message.len() {
        decompressed_length += (message.len() - str_index) as u64;
    }

    decompressed_length
}

fn decompress_string_v2(message: &str, marker_regex: &Regex) -> u64 {
    let mut decompressed_length: u64 = 0;
    let mut str_index: usize = 0;

    while let Some(matched) = marker_regex.find(&message[str_index..]) {
        let captured = marker_regex.captures(&message[str_index..]).unwrap();

        decompressed_length += matched.start() as u64;

        let number_of_chars: usize = (&captured[1]).parse().unwrap();
        let number_of_repeats: u64 = (&captured[2]).parse().unwrap();

        str_index += matched.end() + number_of_chars;

        decompressed_length +=
            number_of_repeats *
            decompress_string_v2(&message[(str_index - number_of_chars)..str_index], marker_regex);
    }

    if str_index < message.len() {
        decompressed_length += (message.len() - str_index) as u64;
    }

    decompressed_length
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decompress_v1() {
        let test_data = vec![Message::new("ADVENT").decompress_v1(),
                             Message::new("A(1x5)BC").decompress_v1(),
                             Message::new("(3x3)XYZ").decompress_v1(),
                             Message::new("A(2x2)BCD(2x2)EFG").decompress_v1(),
                             Message::new("(6x1)(1x3)A").decompress_v1(),
                             Message::new("X(8x2)(3x3)ABCY").decompress_v1()];

        assert_eq!(test_data, vec![6, 7, 9, 11, 6, 18]);
    }

    #[test]
    fn test_decompress_v2() {
        let test_data = vec![Message::new("(3X3)XYZ").decompress_v2(),
                 Message::new("X(8x2)(3x3)ABCY").decompress_v2(),
                 Message::new("(27x12)(20x12)(13x14)(7x10)(1x12)A").decompress_v2(),
                 Message::new("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
                     .decompress_v2()];

        assert_eq!(test_data, vec![9, 20, 241920, 445]);
    }
}