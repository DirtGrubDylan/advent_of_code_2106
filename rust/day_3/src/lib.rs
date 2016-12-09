use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use triangle::Triangle;

pub mod triangle {
    #[derive(Debug, PartialEq)]
    pub struct Triangle(pub i32, pub i32, pub i32);

    impl Triangle {
        pub fn is_valid(&self) -> bool {
            (self.0 + self.1 > self.2) && (self.0 + self.2 > self.1) &&
            (self.1 + self.2 > self.0)
        }
    }
}

pub fn read_row_major_triangles_from_file<P>(file_name: &P)
        -> Result<Vec<Triangle>, String> where P: AsRef<Path> {
    let data_file: File = File::open(file_name).map_err(|e| e.to_string())?;
    let mut data: Vec<Triangle> = Vec::new();

    for line in io::BufReader::new(data_file).lines() {
        let triangle_sides: String = line.map_err(|e| e.to_string())?;
        let triangle_sides: Vec<i32> =
            triangle_sides.split_whitespace().collect::<Vec<&str>>().iter()
                          .map(|&s| s.parse::<i32>().unwrap()).collect();

        let triangle: Triangle = Triangle(
            triangle_sides[0], triangle_sides[1], triangle_sides[2]);

        data.push(triangle);
    }

    Ok(data)
}

pub fn read_column_major_triangles_from_file<P>(file_name: &P)
        -> Result<Vec<Triangle>, String> where P: AsRef<Path> {
    let mut data: Vec<Triangle> = Vec::new();
    let mut triangle_sides: Vec<i32> = Vec::new();
    let data_file: File = File::open(file_name).map_err(|e| e.to_string())?;

    for line in io::BufReader::new(data_file).lines() {
        let temp_sides: String = line.map_err(|e| e.to_string())?;

        let temp_sides: Vec<i32> =
            temp_sides.split_whitespace().collect::<Vec<&str>>().iter()
                      .map(|&s| s.parse::<i32>().unwrap()).collect();

        for triangle_side in temp_sides {
            triangle_sides.push(triangle_side);
        }

        if triangle_sides.len() == 9 {
            for index in 0..3 {
                data.push(
                    Triangle(
                        triangle_sides[index], triangle_sides[index + 3],
                        triangle_sides[index + 6]))
            }

            triangle_sides.clear();
        }
    }

    Ok(data)
}

pub fn number_of_valid_row_major_triangles_in_file<P>(file_name: &P)
        -> Result<u32, String> where P: AsRef<Path> {
    let triangles: Vec<Triangle> =
        read_row_major_triangles_from_file(file_name)?;
    let mut number_valid: u32 = 0;

    for triangle in triangles {
        if triangle.is_valid() {
            number_valid += 1;
        }
    }

    Ok(number_valid)
}

pub fn number_of_valid_column_major_triangles_in_file<P>(file_name: &P)
        -> Result<u32, String> where P: AsRef<Path> {
    let triangles: Vec<Triangle> =
        read_column_major_triangles_from_file(file_name)?;
    let mut number_valid: u32 = 0;

    for triangle in triangles {
        if triangle.is_valid() {
            number_valid += 1;
        }
    }

    Ok(number_valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use triangle::Triangle;
    use std::path::PathBuf;

    #[test]
    fn test_load_row_major_triangles() {
        let mut data_path: PathBuf = env::current_dir().unwrap();
        data_path.push("data");
        data_path.push("input.txt");

        let data: Vec<Triangle> =
            read_row_major_triangles_from_file(&data_path).unwrap();

        let answer:Triangle = Triangle(775, 785, 361);

        assert_eq!(data[0], answer);
    }

    #[test]
    fn test_load_column_major_triangles() {
        let mut data_path: PathBuf = env::current_dir().unwrap();
        data_path.push("data");
        data_path.push("input.txt");

        let data: Vec<Triangle> =
            read_column_major_triangles_from_file(&data_path).unwrap();

        let answer:Triangle = Triangle(38, 463, 482);

        assert_eq!(data[4], answer);
    }

    #[test]
    fn test_triangle_is_valid() {
        let data:Triangle = Triangle(775, 785, 361);

        assert!(data.is_valid());
    }

    #[test]
    #[should_panic]
    fn test_triangle_is_not_valid() {
        let data: Triangle = Triangle(622, 375, 125);

        assert!(data.is_valid());
    }
}