use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

pub mod keypad {
    pub struct KeyPad {
        pub keys: String,
        pub width: usize,
    }

    impl KeyPad {
        pub fn pad(&self) -> Result<Vec<Vec<char>>, String> {
            let mut key_pad: Vec<Vec<char>> = Vec::new();
            let mut key_pad_row: Vec<char> = Vec::new();

            if self.keys.len() % self.width != 0 {
                return Err(
                    "Keys in KeyPad does not form a rectagle!".to_owned());
            }

            for key in self.keys.chars() {
                key_pad_row.push(key);

                if key_pad_row.len() == self.width {
                    key_pad.push(key_pad_row.clone());
                    key_pad_row.clear();
                }
            }

            Ok(key_pad)
        }
    }
}

pub fn read_instructions<P>(file_name: &P)
        -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let data_file: File = File::open(file_name)?;

    Ok(io::BufReader::new(data_file).lines())
}

pub fn interpret_instruction(
        instruction: &String, keypad_pad: &Vec<Vec<char>>,
        starting_position: &[usize; 2]) -> Result<[usize; 2], String> {
    let mut current_position: [usize; 2] = *starting_position;

    let num_rows: usize = keypad_pad.len();
    let num_cols: usize = keypad_pad[0].len();

    if num_rows <= current_position[0] {
        return Err("Bad row position!".to_owned());
    } else if num_cols <= current_position[1] {
        return Err("Bad row position!".to_owned());
    }

    for symbol in instruction.chars() {
        match symbol {
            'U' => {
                current_position[0] =
                    current_position[0].checked_sub(1).unwrap_or(0);

                if keypad_pad[current_position[0]][current_position[1]] == '_' {
                    current_position[0] += 1;
                }
            },
            'D' => {
                if current_position[0] + 1 < num_rows {
                    current_position[0] += 1;
                }

                if keypad_pad[current_position[0]][current_position[1]] == '_' {
                    current_position[0] -= 1;
                }
            },
            'L' => {
                current_position[1] =
                    current_position[1].checked_sub(1).unwrap_or(0);

                if keypad_pad[current_position[0]][current_position[1]] == '_' {
                    current_position[1] += 1;
                }
            },
            'R' => {
                if current_position[1] + 1 < num_cols {
                    current_position[1] += 1;
                }

                if keypad_pad[current_position[0]][current_position[1]] == '_' {
                    current_position[1] -= 1;
                }
            },
            _ => return Err("Bad symbol in instruction: {}!".to_owned()),
        };
    }

    Ok(current_position)
}

pub fn interpret_instructions(
        instructions: &Vec<String>, key_pad: &keypad::KeyPad,
        start_position: [usize; 2]) -> Result<String, String> {
    let keypad_pad: Vec<Vec<char>> = key_pad.pad()?;
    let mut current_position: [usize; 2] = start_position;
    let mut answer: String = String::new();

    for instruction in instructions {
        current_position = interpret_instruction(
            &instruction, &keypad_pad, &current_position)?;

        answer.push(keypad_pad[current_position[0]][current_position[1]]);
    }

    Ok(answer)
}

pub fn buffer_lines_to_string_vec(
    lines: io::Lines<io::BufReader<File>>) -> Result<Vec<String>, String> {
    let mut output: Vec<String> = Vec::new();

    for line in lines {
        output.push(line.map_err(|e| e.to_string())?);
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::fs::File;
    use keypad::KeyPad;

    #[test]
    fn test_read_instructions() {
        let path: &str =
            "/home/dhicks/Documents/advent_of_code_2106/rust/day_2/data/\
            input.txt";

        let answer: &str =
            "RRLLRLLRULLRUUUDRDLDDLLLDDDDDUUURRRRUUDLRULURRRDRUDRUUDDRUDLLLRLD\
            DDUDRDDRRLLLLRLRLULUURDRURRUULDRRDUDURRUURURDLURULLDUDRDLUUUUDDUR\
            RLLLUDLDLRDRRRDULLDLDULLDRLDLDURDLRRULLDDLDRLLLUDDLLRDURULLDDDDDU\
            URURLRLRRDUURUULRLLLULLRLULLUUDRRLLDURLDDDDULUUDLUDDDULRLDURDDRUU\
            DRRUUURLLLULURUDRULDRDUDUDRRDDULRURLLRRLRRLLDLULURDRDRULDRDRURUDL\
            LRRDUUULDDDUURDLULDLRLLURRURLLUDURDDRUDRDLLLLDLRLDLDDRDRRDUUULLUU\
            LRRDLURLDULLDLDUUUULLLDRURLRULLULRLULUURLLRDDRULDULRLDRRURLURUDLR\
            RRLUDLDUULULLURLDDUDDLLUDRUDRLDUDURRRRLRUUURLUDDUDURDUDDDLLRLRDDU\
            RDRUUDUDRULURLRLDRULDRRLRLDDDRDDDRLDUDRLULDLUDLRLRRRLRDULDDLRRDDL\
            DDULDLLDU";

        let data: io::Result<io::Lines<io::BufReader<File>>> =
            read_instructions(&path);

        assert_eq!(data.unwrap().nth(0).unwrap().unwrap(), answer);
    }

    #[test]
    fn test_keypad_pad() {
        let test_keypad: KeyPad = KeyPad {
            keys: "123456789".to_owned(), width: 3 };

        let answer: Vec<Vec<char>> =
            vec![vec!['1', '2', '3'], vec!['4', '5', '6'], vec!['7', '8', '9']];

        assert_eq!(test_keypad.pad().unwrap(), answer);
    }

    #[test]
    fn test_interpret_instruction() {
        let test_keypad: KeyPad = KeyPad {
            keys: "123456789".to_owned(), width: 3 };
        let test_instruction: String = "ULL".to_owned();
        let mut test_position: [usize; 2] = [1, 1];

        test_position =
            interpret_instruction(
                &test_instruction, &test_keypad.pad().unwrap(), &test_position)
                .unwrap();

        assert_eq!(test_position, [0, 0]);
        assert_eq!(
            test_keypad.pad().unwrap()[test_position[0]][test_position[1]],
            '1');
    }

    #[test]
    fn test_interpret_instructions() {
        unimplemented!();
    }
}