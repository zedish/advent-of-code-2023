use std::io;
use regex::Regex;
use memoize::memoize;
use crate::utils;
use std::thread;
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day12_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,0);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day12_2_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let num_threads = 4;
    let contents = utils::read_file(input)?;
    let mut part1 = 0;
    let mut vals = vec![];
    for line in contents.lines(){
        let tmp = line.to_owned();
        let val = thread::spawn(move || process_line(&tmp));
        vals.push(val);
    }

    for val in vals{
        part1+=val.join().unwrap();
    }

    Ok((part1,0))
}

fn process_line(line: &str) -> i64{
        let line_str = format!(".{}.",line.split_whitespace().nth(0).unwrap());
        let mut cur_total: i64 = 0;
        let mut line_vec: Vec<char> = line_str
                                    .split_whitespace()
                                    .nth(0).unwrap()
                                    .chars().map(|c|{
                                    if c == '#'{cur_total +=1;}
                                    c
                                    }).collect();

        let mut regex_str: String = line.split_whitespace().nth(1).unwrap().split(',').map(|val|format!(r#"(#{{{}}})\.\.*"#,val)).collect::<Vec<String>>().join("");
        let mut total: i64 = 0;
        line.split_whitespace().nth(1).unwrap().split(',').for_each(|val| total += val.parse::<i64>().unwrap());
        regex_str = format!("\\.{}",regex_str);
        let regex_val = Regex::new(&regex_str).unwrap();
        replace_elem(&mut line_vec, regex_val,total,cur_total)
}
// #[memoize]
// fn test(input: Vec<char>,  total: i64, cur_total:i64) -> i64{0}
fn replace_elem(input: &mut Vec<char>, regex_val: Regex, total: i64, cur_total:i64) -> i64{
    let mut result = 0;
    if let Some(index) = input.iter().position(|&c| c == '?') {
        *input.get_mut(index).unwrap() = '.';
        result += replace_elem(&mut (*input).clone(), regex_val.clone(),total,cur_total);
        if cur_total < total{
            *input.get_mut(index).unwrap() = '#';
            result += replace_elem(&mut (*input).clone(),regex_val,total,cur_total+1);
        }
    }

    else {
        let tmp: String = input.iter().collect();
        if regex_val.is_match(&tmp){
            // println!("{:?}",input);
            result += 1
        }
    }
    result

}


