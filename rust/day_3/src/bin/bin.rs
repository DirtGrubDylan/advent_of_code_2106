extern crate day_3;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut data_path: PathBuf = env::current_dir().unwrap();
    data_path.push("data");
    data_path.push("input.txt");

    let number_of_valid_row_major_triangles_in_file: u32 =
        day_3::number_of_valid_row_major_triangles_in_file(&data_path).unwrap();

    println!("Answer 1: {}", number_of_valid_row_major_triangles_in_file);
}
