use std::io;
use std::fs::File;
use std::io::BufRead;
use std::collections::BTreeMap;


pub fn load_messages_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let data_file = File::open(file_path)?;
    let mut data = Vec::new();

    for line in io::BufReader::new(data_file).lines() {
        data.push(line?);
    }

    Ok(data)
}


pub fn message_from_most_used(repeating_message_signal: &Vec<String>) -> String {
    let mut column_char_counters: Vec<BTreeMap<char, u32>> = Vec::new();

    column_char_counters.resize(repeating_message_signal[0].len(), BTreeMap::new());

    for message in repeating_message_signal {
        for (counter, character) in column_char_counters.iter_mut().zip(message.chars()) {
            *counter.entry(character).or_insert(0) += 1;
        }
    }

    let mut message = String::new();

    for counter in column_char_counters.into_iter() {
        message.push(*counter.iter().max_by_key(|char_count| char_count.1).unwrap().0);
    }

    message
}


pub fn message_from_least_used(repeating_message_signal: &Vec<String>) -> String {
    let mut column_char_counters: Vec<BTreeMap<char, u32>> = Vec::new();

    column_char_counters.resize(repeating_message_signal[0].len(), BTreeMap::new());

    for message in repeating_message_signal {
        for (counter, character) in column_char_counters.iter_mut().zip(message.chars()) {
            *counter.entry(character).or_insert(0) += 1;
        }
    }

    let mut message = String::new();

    for counter in column_char_counters.into_iter() {
        message.push(*counter.iter().min_by_key(|char_count| char_count.1).unwrap().0);
    }

    message
}


#[cfg(test)]
mod name {
    use super::*;

    #[test]
    fn test_load_messages_from_file() {
        let test_data = vec!["eedadn".to_string(),
                             "drvtee".to_string(),
                             "eandsr".to_string(),
                             "raavrd".to_string(),
                             "atevrs".to_string(),
                             "tsrnev".to_string(),
                             "sdttsa".to_string(),
                             "rasrtv".to_string(),
                             "nssdts".to_string(),
                             "ntnada".to_string(),
                             "svetve".to_string(),
                             "tesnvt".to_string(),
                             "vntsnd".to_string(),
                             "vrdear".to_string(),
                             "dvrsen".to_string(),
                             "enarar".to_string()];

        assert_eq!(
            load_messages_from_file(
                "/home/dhicks/Documents/advent_of_code_2106/python/tests/test_data/\
                test_day6_data.txt").unwrap(),
            test_data);
    }


    #[test]
    fn test_message_from_most_used() {
        let test_data = load_messages_from_file(
            "/home/dhicks/Documents/advent_of_code_2106/python/tests/test_data/test_day6_data.txt")
            .unwrap();

        assert_eq!(message_from_most_used(&test_data), "easter".to_string());
    }


    #[test]
    fn test_message_from_least_used() {
        let test_data = load_messages_from_file(
            "/home/dhicks/Documents/advent_of_code_2106/python/tests/test_data/test_day6_data.txt")
            .unwrap();

        assert_eq!(message_from_least_used(&test_data), "advent".to_string());
    }
}
