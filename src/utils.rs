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
