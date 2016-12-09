use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use triangle::Triangle;

pub mod triangle {
    #[derive(Debug)]
    pub struct Triangle {
        pub side_a: i32,
        pub side_b: i32,
        pub side_c: i32
    }

    impl Triangle {
        pub fn is_valid(&self) -> bool {
            (self.side_a + self.side_b > self.side_c) &&
            (self.side_a + self.side_c > self.side_b) &&
            (self.side_b + self.side_c > self.side_a)
        }
    }
}

pub fn read_triangles_from_file<P>(file_name: &P)
        -> Result<Vec<Vec<Triangle>>, String> where P: AsRef<Path> {
    let data_file: File = File::open(file_name).map_err(|e| e.to_string())?;
    let mut data: Vec<Triangle> = Vec::new();

    for line in io::BufReader::new(data_file).lines() {
        let triangle_sides: String = line.map_err(|e| e.to_string())?;
        let triangle_sides: Vec<i32> =
            triangle_sides.split_whitespace().collect::<Vec<&str>>().iter()
                          .map(|&s| s.parse::<i32>().unwrap()).collect();
        let triangle: Triangle = Triangle {
            side_a: triangle_sides[0],
            side_b: triangle_sides[1],
            side_c: triangle_sides[2]
        };

        data.push(triangle);
    }

    Ok(data)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use triangle::Triangle;
    use std::path::PathBuf;

    #[test]
    fn test_load_triangles() {
        let mut data_path: PathBuf = env::current_dir().unwrap();
        data_path.push("data");
        data_path.push("input.txt");

        let data: Vec<Vec<i32>> = read_triangles_from_file(&data_path).unwrap();

        let answer: Triangle = Triangle {
            side_a: data[0][0],
            side_b: data[0][1],
            side_c: data[0][2]
        };

        assert_eq!(data[0], answer);
    }
}