use std::fs::File;
use std::io::{self, Read};
use std::env;
use std::path::PathBuf;

pub fn read_file(input: &str) -> Result<String, io::Error> {
    let path_head = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let relative_path = format!("{}{}","src/input/",input);
    let file_path = path_head.join(relative_path);
    
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}


pub fn convert_to_int(input: &str) -> i32 {
    match input.parse::<i32>() {
        Ok(number) => {
            number
        }
        Err(_) => {
            println!("Failed to parse the string \"{}\" as an int", input);
            0
        }
    }
}

pub fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() {
        return vec![];
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = vec![vec!['.'; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = matrix[i][j];
        }
    }

    result
}
