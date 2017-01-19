extern crate regex;

mod ipv7;

use std::io;
use std::fs::File;
use std::io::BufRead;

pub use ipv7::IPv7;

pub fn load_ipv7s_from_file(file_path: &str) -> io::Result<Vec<IPv7>> {
    let mut data: Vec<IPv7> = Vec::new();
    let data_file: File = File::open(file_path)?;


    for line in io::BufReader::new(data_file).lines() {
        data.push(IPv7::new(line?));
    }

    Ok(data)
}


pub fn number_of_tls_supported(ipv7_addresses: &Vec<IPv7>) -> u32 {
    ipv7_addresses.iter().fold(0, |accumulator, ipv7| {
        if ipv7.supports_tls() {
            accumulator + 1
        } else {
            accumulator
        }
    })
}


pub fn number_of_ssl_supported(ipv7_addresses: &Vec<IPv7>) -> u32 {
    ipv7_addresses.iter().fold(0, |accumulator, ipv7| {
        if ipv7.supports_ssl() {
            accumulator + 1
        } else {
            accumulator
        }
    })
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_ipv7s_from_file() {
        let test_data: Vec<IPv7> = vec![IPv7::new("abba[mnop]qrst".to_string()),
                                        IPv7::new("abcd[bddb]xyyx".to_string()),
                                        IPv7::new("aaaa[qwer]tyui".to_string()),
                                        IPv7::new("ioxxoj[asdfgh]zxcvbn".to_string()),
                                        IPv7::new("aba[bab]xyz".to_string()),
                                        IPv7::new("xyx[xyx]xyx".to_string()),
                                        IPv7::new("aaa[kek]eke".to_string()),
                                        IPv7::new("zazbz[bzb]cdb".to_string())];

        assert_eq!(
            load_ipv7s_from_file("/home/dhicks/Documents/advent_of_code_2106/python/tests/\
                                 test_data/test_day7_data.txt").unwrap(),
            test_data);
    }


    #[test]
    fn test_number_of_tls_supported() {
        let test_data: Vec<IPv7> = vec![IPv7::new("abba[mnop]qrst".to_string()),
                                        IPv7::new("abcd[bddb]xyyx".to_string()),
                                        IPv7::new("aaaa[qwer]tyui".to_string()),
                                        IPv7::new("ioxxoj[asdfgh]zxcvbn".to_string()),
                                        IPv7::new("aba[bab]xyz".to_string()),
                                        IPv7::new("xyx[xyx]xyx".to_string()),
                                        IPv7::new("aaa[kek]eke".to_string()),
                                        IPv7::new("zazbz[bzb]cdb".to_string())];

        assert_eq!(number_of_tls_supported(&test_data), 2);
    }


    #[test]
    fn test_number_of_ssl_supported() {
        let test_data: Vec<IPv7> = vec![IPv7::new("abba[mnop]qrst".to_string()),
                                        IPv7::new("abcd[bddb]xyyx".to_string()),
                                        IPv7::new("aaaa[qwer]tyui".to_string()),
                                        IPv7::new("ioxxoj[asdfgh]zxcvbn".to_string()),
                                        IPv7::new("aba[bab]xyz".to_string()),
                                        IPv7::new("xyx[xyx]xyx".to_string()),
                                        IPv7::new("aaa[kek]eke".to_string()),
                                        IPv7::new("zazbz[bzb]cdb".to_string())];

        assert_eq!(number_of_ssl_supported(&test_data), 3);
    }
}