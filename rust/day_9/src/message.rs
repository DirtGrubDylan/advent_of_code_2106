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
    message.to_string()
}