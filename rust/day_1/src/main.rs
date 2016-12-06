use std::env;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

fn main() {
    let mut temp: Vec<String> = Vec::new();
    let mut data_path: std::path::PathBuf = env::current_dir().unwrap();
    data_path.push("data");
    data_path.push("input.txt");

    println!("The path is: {}", data_path.display());

    let data: std::io::Lines<std::io::BufReader<File>> =
        data_from_file(&data_path).unwrap();

    for data_line in data {
        temp = data_line.unwrap().split(',').map(|s| s.to_owned()).collect();
    }

    println!("{:?}", temp);
}

fn data_from_file<P: AsRef<Path>>(file_name: &P) ->
        Result<std::io::Lines<std::io::BufReader<File>>, std::io::Error> {
    let data_file: File = File::open(file_name)?;

    Ok(std::io::BufReader::new(data_file).lines())
}