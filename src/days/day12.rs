use std::io;
use memoize::memoize;
use crate::utils;
use std::thread;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day12_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,21);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day12_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,525152);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_solve() {
        let result = do_puzzle("day12_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,7260);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_solve() {
        let result = do_puzzle("day12_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,1909291258644);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day12_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let mut part1 = 0;
    let mut part2 = 0;
    let mut vals = vec![];
    let mut vals2 = vec![];
    for line in contents.lines(){
        let tmp = line.to_owned();
        let val = thread::spawn(move || process_line(&tmp));
        vals.push(val);
    }

    for val in vals{
        part1+=val.join().unwrap();
    }
    if true{
        for line in contents.lines(){
            let mut mod_line = String::new();
            mod_line.push_str(line.split_whitespace().nth(0).unwrap());
            for _ in 1..5{
                mod_line.push_str("?");
                mod_line.push_str(line.split_whitespace().nth(0).unwrap());
            }
            mod_line.push_str(" ");
            mod_line.push_str(line.split_whitespace().nth(1).unwrap());
            for _ in 1..5{
                mod_line.push_str(",");
                mod_line.push_str(line.split_whitespace().nth(1).unwrap());
            }
            let tmp = mod_line.to_owned();
            let val = thread::spawn(move || process_line(&tmp));
            vals2.push(val);
        }
        for val in vals2{
            part2+=val.join().unwrap();
        }
    }
    Ok((part1,part2))
}

fn process_line(line: &str) -> i64{
        let mut line_vec: Vec<char> = line
                                    .split_whitespace()
                                    .nth(0).unwrap()
                                    .chars().map(|c|{c})
                                    .collect();
        line_vec.push('.');
        let check_vec:Vec<i32> = line
                                .split_whitespace()
                                .nth(1).unwrap()
                                .split(',').map(|val| 
                                val.parse::<i32>()
                                .unwrap()).collect();
        
        replace_elem(line_vec, check_vec)
}
#[memoize]
fn replace_elem(input: Vec<char>, check_vec: Vec<i32> ) -> i64{
    let mut result = 0;
    let mut count = 0;
    if input.len() == 0 && check_vec.len()==0{
        result += 1;
    }
    for (index, c) in input.iter().enumerate(){
        match count {
            0 => {match c {
                    '.' => {
                        result+=replace_elem(input[index+1..].to_vec(), check_vec.clone());
                        break;
                    },
                    '#' => {
                        //if we don't have anymore to match but 
                        //find one don't add it
                        if check_vec.len() == 0{
                            break;
                        }
                        count += 1;
                        },
                    '?' => {
                        // if we assume it was a '.'
                        result+=replace_elem(input[index+1..].to_vec(), check_vec.clone());
                        //if we assume it was a '#'
                        count += 1;
                    },
                     _  => println!("How did this happen?"),
                }    
            }
            _ => match c {
                    '.' => {
                        if !correct_count(&count, &check_vec){
                            break;
                        }
                        result+=replace_elem(input[index+1..].to_vec(), check_vec[1..].to_vec());
                        break;
                    },
                    '#' => {
                        count += 1;
                        if !valid_count(&count, &check_vec){
                            break;
                        }
                    },
                    '?' => {
                        //if we hit a ? and the count is correct
                        // then the ? is a . and continue
                        if correct_count(&count, &check_vec){
                            result+=replace_elem(input[index+1..].to_vec(), check_vec[1..].to_vec());
                            break;
                        }
                        //if we hit a ? and don't have enough yet
                        //then it is a # and continue  
                        else {
                            count += 1;
                        }
                    },
                     _  => println!("How did this happen?"),
                }
        };
    }
    return result;
}

fn valid_count(input:&i32, check_vec:&Vec<i32>) -> bool{
    if let Some(val) = check_vec.get(0){
        if val < input { //we have found too many #
        return false;
        }
    }
    return true;
}
fn correct_count(input:&i32, check_vec:&Vec<i32>) -> bool{
    if let Some(val) = check_vec.get(0){
        if val == input { //we have found too many #
        return true;
        }
    }
    return false;
}
