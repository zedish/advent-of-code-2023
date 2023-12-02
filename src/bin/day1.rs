use std::fs::File;
use std::io::{self, Read};
use std::env;
use std::path::PathBuf;
use regex::Regex;

fn main() -> io::Result<()> {
    let _ = do_puzzle("day1_1_0.txt",1);
    let _ = do_puzzle("day1_1.txt",1);
    let _ = do_puzzle("day1_2_0.txt",2);
    let _ = do_puzzle("day1_1.txt",2);

    Ok(())
}

fn read_file(input: &str) -> Result<String, io::Error> {
    let path_head = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let relative_path = format!("{}{}","src/input/",input);
    let file_path = path_head.join(relative_path);
    
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn do_puzzle(input: &str, part: i32) -> Result<i32, io::Error>{
    println!("Solving puzzle for file {} using part {}", input,part);
    
    let contents = read_file(input)?;
    

    let mut result = 0;
    let re_first = Regex::new(r"\d").unwrap();
    for line in contents.lines(){
        let tmp_line;
        if part == 1 {
            tmp_line = line.to_string();
        } else if part == 2 {
            tmp_line = process_line(line);
        } else {
            tmp_line = line.to_string();
        }
        let processed_line = tmp_line.as_str();
       
        if let Some(num) = re_first.find(processed_line) {
            result += convert_to_int(num.as_str()) * 10;
        } else {
            println!("No first int found in str \"{}\"",line);
        }

        let mut last_digit: Option<&str> = None;    
        for digit in re_first.find_iter(processed_line) {
            last_digit = Some(digit.as_str());
        }
        result += convert_to_int(last_digit.unwrap_or("0"));
    }
    println!("Result:{}",result);
    Ok(result)
}


fn convert_to_int(input: &str) -> i32 {
    match input.parse::<i32>() {
        Ok(number) => {
            number
        }
        Err(_) => {
            println!("Filaed to parse the string \"{}\" as an int", input);
            0
        }
    }
}

fn process_line(input: &str) -> String {
    let mut tmp_input = String::from(input);
    let numbers: Vec<(&str, &str)> = vec![
        ("zero",  "0"),
        ("one",   "1"),
        ("two",   "2"),
        ("three", "3"),
        ("four",  "4"),
        ("five",  "5"),
        ("six",   "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine",  "9")
    ];
    let mut found;
    let mut found_index = 999;
    let mut found_value = String::new();
    loop{
        found = false;
        for (word, value) in &numbers {
            if let Some(index) = tmp_input.find(word){
                if index < found_index {
                    found_index = index;
                    found_value = value.to_string();
                }
                found = true;
            }
        }
        if !found{
            break;
        }
        tmp_input.replace_range(found_index..found_index+1, found_value.as_str());
        found_index = 999;
    }
    tmp_input
}

