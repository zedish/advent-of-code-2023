use std::io;
use regex::Regex;

use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_letter_parsing() {
        assert_eq!(process_line("1abc2"),"1abc2");
        assert_eq!(process_line("pqr3stu8vwx"),"pqr3stu8vwx");
        assert_eq!(process_line("a1b2c3d4e5f"),"a1b2c3d4e5f");
        assert_eq!(process_line("treb7uchet"),"treb7uchet");
        assert_eq!(process_line("two1nine"),"2wo19ine");
        assert_eq!(process_line("eightwothree"),"8igh2wo3hree");
        assert_eq!(process_line("abcone2threexyz"),"abc1ne23hreexyz");
        assert_eq!(process_line("xtwone3four"),"x2w1ne34our");
        assert_eq!(process_line("4nineeightseven2"),"49ine8ight7even2");
        assert_eq!(process_line("zoneight234"),"z1n8ight234");
        assert_eq!(process_line("7pqrstsixteen"),"7pqrst6ixteen");
    }
    #[test]
    fn test_part2_examples() {
        assert_eq!(find_digits("two1nine",2),29);
        assert_eq!(find_digits("eightwothree",2),83);
        assert_eq!(find_digits("abcone2threexyz",2),13);
        assert_eq!(find_digits("xtwone3four",2),24);
        assert_eq!(find_digits("4nineeightseven2",2),42);
        assert_eq!(find_digits("zoneight234",2),14);
        assert_eq!(find_digits("7pqrstsixteen",2),76);
    }
    #[test]
    fn test_part1_examples() {
        assert_eq!(find_digits("1abc2",1),12);
        assert_eq!(find_digits("pqr3stu8vwx",1),38);
        assert_eq!(find_digits("a1b2c3d4e5f",1),15);
        assert_eq!(find_digits("treb7uchet",1),77);
    }
}

pub fn solve() -> io::Result<()> {
    let _ = do_puzzle("day1_1.txt",1);
    let _ = do_puzzle("day1_1.txt",2);
    Ok(())
}

fn do_puzzle(input: &str, part: i32) -> Result<i32, io::Error>{
    println!("Solving puzzle for file {} using part {}", input,part);
    
    let contents = utils::read_file(input)?;

    let mut result = 0;
    for line in contents.lines(){
        result += find_digits(line, part);
    }
    println!("Result:{}",result);
    Ok(result)
}

fn find_digits(line: &str, part: i32) -> i32{
    let re_first = Regex::new(r"\d").unwrap();
    let tmp_line;
    let mut result = 0;
    if part == 1 {
        tmp_line = line.to_string();
    } else if part == 2 {
        tmp_line = process_line(line);
    } else {
        tmp_line = line.to_string();
    }
    let processed_line = tmp_line.as_str();
       
    if let Some(num) = re_first.find(processed_line) {
        result += utils::convert_to_int(num.as_str()) * 10;
    } else {
        println!("No first int found in str \"{}\"",line);
    }

    let mut last_digit: Option<&str> = None;    
    for digit in re_first.find_iter(processed_line) {
        last_digit = Some(digit.as_str());
    }
    result += utils::convert_to_int(last_digit.unwrap_or("0"));

    result
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

