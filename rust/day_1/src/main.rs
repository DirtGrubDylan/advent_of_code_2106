use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut data_path: PathBuf = env::current_dir().unwrap();
    data_path.push("data");
    data_path.push("input.txt");

    let directions: Vec<String> = data_from_file(&data_path).unwrap();

    let longest_distance: i32 = longest_block_distance(&directions).unwrap();

    let distance_to_reoccurance: i32 =
        first_reoccuring_block_distance(&directions).unwrap();

    println!("Answer 1: {}", longest_distance);
    println!("Answer 2: {}", distance_to_reoccurance);
}

pub fn data_from_file<P>(file_name: &P) -> Result<Vec<String>, std::io::Error>
        where P: AsRef<Path> {
    let mut data_file: File = File::open(file_name)?;
    let mut data_line: String = String::new();

    data_file.read_to_string(&mut data_line)?;

    let data: Vec<String> = data_line.trim()
                                     .split(", ")
                                     .map(|str_literal| str_literal.to_owned())
                                     .collect();

    Ok(data)
}

pub fn longest_block_distance(directions: &Vec<String>) -> Result<i32, String> {
    let mut index: i32 = 0;
    let mut ew_parity: i32 = 1;
    let mut ns_parity: i32 = 1;

    let mut direction_parity_table: HashMap<char, i32> = HashMap::new();
    direction_parity_table.insert('R', 1);
    direction_parity_table.insert('L', -1);

    let mut travel_location: [i32; 2] = [0, 0];

    for direction in directions {
        let direction_char: char =
            direction
                .chars().nth(0).ok_or("Direction is empty!".to_owned())?;
        let direction_parity: &i32 =
            direction_parity_table
                .get(&direction_char)
                .ok_or("Direction not in parity table!".to_owned())?;
        let distance: i32 =
            (&direction[1..]).parse::<i32>().map_err(|e| e.to_string())?;

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

    Ok(travel_location[0].abs() + travel_location[1].abs())
}

pub fn first_reoccuring_block_distance(directions: &Vec<String>)
         -> Result<i32, String> {
    let mut index: i32 = 0;
    let mut ew_parity: i32 = 1;
    let mut ns_parity: i32 = 1;

    let mut direction_parity_table: HashMap<char, i32> = HashMap::new();
    direction_parity_table.insert('R', 1);
    direction_parity_table.insert('L', -1);

    let mut travel_location: [i32; 2] = [0, 0];

    let mut locations_visited: HashSet<[i32; 2]> = HashSet::new();
    locations_visited.insert(travel_location);

    for direction in directions {
        let direction_char: char =
            direction
                .chars().nth(0).ok_or("Direction is empty!".to_owned())?;
        let direction_parity: &i32 =
            direction_parity_table
                .get(&direction_char)
                .ok_or("Direction not in parity table!".to_owned())?;
        let distance: i32 =
            (&direction[1..]).parse::<i32>().map_err(|e| e.to_string())?;

        if index % 2 == 0 {
            if ns_parity == 1 {
                ew_parity = *direction_parity;
            } else {
                ew_parity = -1 * (*direction_parity);
            }

            let mut travel_from: i32 = travel_location[1];

            travel_location[1] += ew_parity * distance;

            let travel_to: i32 = travel_location[1];

            while travel_from != travel_to {
                travel_from += ew_parity;

                let temp_location: [i32; 2] =
                    [travel_location[0], travel_from];

                if locations_visited.contains(&temp_location) {
                    return Ok(temp_location[0].abs() + temp_location[1].abs());
                }

                locations_visited.insert(temp_location);
            }
        } else {
            if ew_parity == 1 {
                ns_parity = -1 * (*direction_parity);
            } else {
                ns_parity = *direction_parity;
            }

            let mut travel_from: i32 = travel_location[0];

            travel_location[0] += ns_parity * distance;

            let travel_to: i32 = travel_location[0];

            while travel_from != travel_to {
                travel_from += ns_parity;

                let temp_location: [i32; 2] =
                    [travel_from, travel_location[1]];

                if locations_visited.contains(&temp_location) {
                    return Ok(temp_location[0].abs() + temp_location[1].abs());
                }

                locations_visited.insert(temp_location);
            }
       }

        index += 1;
    }

    Err("There is no intersection!".to_owned())
}

// Testing
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_data_from_file() {
        let data_string: String = (
            "R1, R3, L2, L5, L2, L1, R3, L4, R2, L2, L4, R2, L1, R1, L2, R3, \
            L1, L4, R2, L5, R3, R4, L1, R2, L1, R3, L4, R5, L4, L5, R5, L3, \
            R2, L3, L3, R1, R3, L4, R2, R5, L4, R1, L1, L1, R5, L2, R1, L2, \
            R188, L5, L3, R5, R1, L2, L4, R3, R5, L3, R3, R45, L4, R4, R72, \
            R2, R3, L1, R1, L1, L1, R192, L1, L1, L1, L4, R1, L2, L5, L3, R5, \
            L3, R3, L4, L3, R1, R4, L2, R2, R3, L5, R3, L1, R1, R4, L2, L3, \
            R1, R3, L4, L3, L4, L2, L2, R1, R3, L5, L1, R4, R2, L4, L1, R3, \
            R3, R1, L5, L2, R4, R4, R2, R1, R5, R5, L4, L1, R5, R3, R4, R5, \
            R3, L1, L2, L4, R1, R4, R5, L2, L3, R4, L4, R2, L2, L4, L2, R5, \
            R1, R4, R3, R5, L4, L4, L5, L5, R3, R4, L1, L3, R2, L2, R1, L3, \
            L5, R5, R5, R3, L4, L2, R4, R5, R1, R4, L3").to_owned();

        let data_vec: Vec<String> = data_string.split(", ")
                                               .map(|s| s.to_owned())
                                               .collect();

        let mut data_path: PathBuf = env::current_dir().unwrap();
        data_path.push("data");
        data_path.push("input.txt");

        assert_eq!(data_from_file(&data_path).unwrap(), data_vec);
    }

    #[test]
    fn test_longest_distance() {
        assert_eq!(
            longest_block_distance(
                &(vec!["R2".to_owned(), "L3".to_owned()])).unwrap(), 5);

        assert_eq!(
            longest_block_distance(
                &(vec!["R2".to_owned(), "R2".to_owned(), "R2".to_owned()]))
                .unwrap(),
            2);

        assert_eq!(
            longest_block_distance(
                &(vec![
                    "R5".to_owned(), "L5".to_owned(), "R5".to_owned(),
                    "R3".to_owned()]))
                .unwrap(),
            12);
    }

    #[test]
    fn test_first_reoccuring_block_distance() {
        let test_vec: Vec<String> =
            vec!["R8".to_owned(), "R4".to_owned(), "R4".to_owned(),
                 "R8".to_owned()];

        assert_eq!(first_reoccuring_block_distance(&test_vec).unwrap(), 4);
    }
}