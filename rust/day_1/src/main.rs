use std::env;
use std::io::Read;
use std::fs::File;

fn main() {
    let mut data_path: std::path::PathBuf = env::current_dir().unwrap();
    data_path.push("data");
    data_path.push("input.txt");

    data_from_file(data_path);
}

fn data_from_file(file_name: std::path::PathBuf) {
    println!("The path is: {}", file_name.display());

    let mut data_file = File::open(file_name).unwrap();

    let mut line = String::new();

    data_file.read_to_string(&mut line).unwrap();

    println!("The data is: {}", line);
}