use std::env;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;

fn main() {
    let mut data_path: std::path::PathBuf = env::current_dir().unwrap();
    data_path.push("data");
    data_path.push("input.txt");

    let directions: Vec<String> = data_from_file(&data_path).unwrap();

    let longest_distance: i32 = longest_block_distance(&directions);

    println!("Answer 1: {}", longest_distance);
}

pub fn data_from_file(file_name: &PathBuf) -> 
        Result<Vec<String>, std::io::Error> {
    let mut data_file: File = File::open(file_name)?;
    let mut data_line: String = String::new();

    data_file.read_to_string(&mut data_line)?;

    let data: Vec<String> = data_line.trim()
                                     .split(", ")
                                     .map(|str_literal| str_literal.to_owned())
                                     .collect();

    Ok(data)
}

pub fn longest_block_distance(directions: &Vec<String>) -> i32 {
    let mut index: i32 = 0;
    let mut ew_parity: i32 = 1;
    let mut ns_parity: i32 = 1;

    let mut direction_parity_table: HashMap<char, i32> = HashMap::new();
    direction_parity_table.insert('R', 1);
    direction_parity_table.insert('L', -1);
    
    let mut travel_location: [i32; 2] = [0, 0];

    for direction in directions {
        let direction_parity: &i32 = direction_parity_table.get(
            &(direction.chars().nth(0).unwrap())).unwrap();
        let distance: i32 = (&direction[1..]).parse().unwrap();

        if index % 2 == 0 {
            if ns_parity == 1 {
                ew_parity = *direction_parity;
            } else {
                ew_parity = -1 * (*direction_parity);
            }

            travel_location[1] += ew_parity * distance;
        } else {
            if ew_parity == 1 {
                ns_parity = -1 * (*direction_parity);
            } else {
                ns_parity = *direction_parity;
            }
            
            travel_location[0] += ns_parity * distance;
       }

        index += 1;
    }

    return travel_location.iter().sum();
}

// Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_distance() {
        assert_eq!(longest_block_distance(&vec!["R2", "L3"]), 5);
    }
}