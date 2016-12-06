use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut data_path: std::path::PathBuf = env::current_dir().unwrap();
    data_path.push("data");
    data_path.push("input.txt");

    let data: Vec<String> = data_from_file(&data_path).unwrap();

    println!("The path is: {}", data_path.display());
    println!("{:?}", data);
}

fn data_from_file<P: AsRef<Path>>(file_name: &P) ->
        Result<Vec<String>, std::io::Error> {
    let mut data_file: File = File::open(file_name)?;
    let mut data_line: String = String::new();

    data_file.read_to_string(&mut data_line)?;

    let data: Vec<String> = data_line.trim()
                                     .split(',')
                                     .map(|str_literal| str_literal.to_owned())
                                     .collect();

    Ok(data)
}