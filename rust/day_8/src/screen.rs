use std::fmt;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Screen {
    pub display: Vec<VecDeque<char>>,
}


impl Screen {
    pub fn new(num_cols: usize, num_rows: usize) -> Screen {
        let mut screen_display = Vec::new();

        for _ in 0..num_rows {
            let mut display_row = VecDeque::new();

            // wait until stable
            // display_row.resize(num_cols, '.');

            for _ in 0..num_cols {
                display_row.push_back(' ');
            }

            screen_display.push(display_row);
        }

        Screen { display: screen_display }
    }


    pub fn rotate_row(&mut self, row: usize, num_rotations: u32) -> () {
        for _ in 0..num_rotations {
            let back = self.display[row].pop_back().unwrap();

            self.display[row].push_front(back);
        }
    }


    pub fn rotate_column(&mut self, column: usize, num_rotations: u32) -> () {
        let mut temp_column = VecDeque::new();

        for row in 0..(self.display.len()) {
            temp_column.push_back(self.display[row][column]);
        }

        for _ in 0..num_rotations {
            let back = temp_column.pop_back().unwrap();

            temp_column.push_front(back);
        }

        for row in 0..(self.display.len()) {
            self.display[row][column] = temp_column[row]
        }
    }


    pub fn fill_rectangle(&mut self, num_cols: usize, num_rows: usize) -> () {
        for row in 0..num_rows {
            for column in 0..num_cols {
                self.display[row][column] = '#';
            }
        }
    }


    pub fn number_of_lit_pixels(&self) -> u32 {
        let mut number_lit = 0;

        for row in &self.display {
            for pixel in row {
                if *pixel == '#' {
                    number_lit += 1;
                }
            }
        }

        number_lit
    }
}


impl fmt::Display for Screen {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.display {
            write!(formatter, "\n{}", line.iter().cloned().collect::<String>())?;
        }

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_new() {
        let test_screen: Vec<VecDeque<char>> =
            vec![vec!['.', '.', '.', '.', '.', '.', '.'].into_iter().collect(),
                 vec!['.', '.', '.', '.', '.', '.', '.'].into_iter().collect(),
                 vec!['.', '.', '.', '.', '.', '.', '.'].into_iter().collect()];

        assert_eq!(Screen::new(7, 3).display, test_screen);
    }


    #[test]
    fn test_rotate_row() {
        let mut test_screen = Screen::new(7, 3);
        let answer: Vec<VecDeque<char>> =
            vec![vec!['.', '.', '.', '.', '#', '.', '#'].into_iter().collect(),
                 vec!['#', '#', '#', '.', '.', '.', '.'].into_iter().collect(),
                 vec!['.', '#', '.', '.', '.', '.', '.'].into_iter().collect()];

        test_screen.fill_rectangle(3, 2);
        test_screen.rotate_column(1, 1);
        test_screen.rotate_row(0, 4);

        assert_eq!(test_screen.display, answer);
    }


    #[test]
    fn test_rotate_column() {
        let mut test_screen = Screen::new(7, 3);
        let answer: Vec<VecDeque<char>> =
            vec![vec!['#', '.', '#', '.', '.', '.', '.'].into_iter().collect(),
                 vec!['#', '#', '#', '.', '.', '.', '.'].into_iter().collect(),
                 vec!['.', '#', '.', '.', '.', '.', '.'].into_iter().collect()];

        test_screen.fill_rectangle(3, 2);
        test_screen.rotate_column(1, 1);

        assert_eq!(test_screen.display, answer);
    }


    #[test]
    fn test_fill_rectangle() {
        let mut test_screen = Screen::new(7, 3);
        let answer: Vec<VecDeque<char>> =
            vec![vec!['#', '#', '#', '.', '.', '.', '.'].into_iter().collect(),
                 vec!['#', '#', '#', '.', '.', '.', '.'].into_iter().collect(),
                 vec!['.', '.', '.', '.', '.', '.', '.'].into_iter().collect()];

        test_screen.fill_rectangle(3, 2);

        assert_eq!(test_screen.display, answer);
    }


    #[test]
    fn test_number_of_lit_pixels() {
        let mut test_screen = Screen::new(7, 3);
        test_screen.fill_rectangle(3, 2);

        assert_eq!(test_screen.number_of_lit_pixels(), 6);
    }
}