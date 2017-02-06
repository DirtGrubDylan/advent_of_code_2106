use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Message {
    pub data: String,
}

impl Message {
    pub fn new(data: &str) -> Message {
        Message { data: data.to_string() }
    }

    pub fn decompress_v1(&self) -> String {
        let marker_finder: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

        decompress_string_v1(&self.data, marker_finder)
    }
    
    pub fn decompress_v2(&self) -> String {
        unimplemented!();
    }
}


fn decompress_string_v1(message: &str, marker_regex: Regex) -> String {
    let mut decompressed_message = String::new();
    let mut str_index: usize = 0;

    while let Some(matched) = marker_regex.find(&message[str_index..]) {
        let captured = marker_regex.captures(&message[str_index..]).unwrap();

        decompressed_message += &message[str_index..(str_index + matched.start())];

        let number_of_chars: usize = (&captured[1]).parse().unwrap();
        let number_of_repeats: usize = (&captured[2]).parse().unwrap();

        str_index += matched.end() + number_of_chars;

        for _ in 0..number_of_repeats {
            decompressed_message += &message[(str_index - number_of_chars)..str_index];
        }
    }

    if str_index < message.len() {
        decompressed_message += &message[str_index.. message.len()];
    }

    decompressed_message
}