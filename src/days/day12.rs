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
    let result = do_puzzle("day12_1.txt"); 
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
    println!("part1:{}",part1);
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
        println!("val:{}",part2);
    }
    Ok((part1,part2))
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

        // let mut regex_str: String = line.split_whitespace().nth(1).unwrap().split(',').map(|val|format!(r#"(#{{{}}})\.\.*"#,val)).collect::<Vec<String>>().join("");
        let mut total: i64 = 0;
        line.split_whitespace().nth(1).unwrap().split(',').for_each(|val| total += val.parse::<i64>().unwrap());
        // regex_str = format!("\\.{}",regex_str);

        let check_vec:Vec<i32> = line.split_whitespace().nth(1).unwrap().split(',').map(|val| val.parse::<i32>().unwrap()).collect();
        
        // let regex_val = Regex::new(&regex_str).unwrap();
        // replace_elem(&mut line_vec, regex_val,total,cur_total)
        replace_elem2(&mut line_vec, check_vec,total,cur_total)
        // test(line_vec,regex_str,total,cur_total)
}
fn replace_elem2(input: &mut Vec<char>, check_vec: Vec<i32>, total: i64, cur_total:i64) -> i64{
    let mut result = 0;
    if let Some(index) = input.iter().position(|&c| c == '?') {
        *input.get_mut(index).unwrap() = '.';
        result += replace_elem2(&mut (*input).clone(), check_vec.clone(),total,cur_total);
        if cur_total < total{
            *input.get_mut(index).unwrap() = '#';
            result += replace_elem2(&mut (*input).clone(),check_vec, total,cur_total+1);
        }
    }

    else {
        // let mut vec_result: Vec<i32> = Vec::new();
        let mut tmp = 0;
        let mut cur_index = 0;
        let mut equal = true;
        for c in input{
            if *c == '#'{
                tmp += 1;
            }
            else{
                if tmp > 0{
                    // vec_result.push(tmp);
                    // if vec_result.len() > check_vec.len(){
                    //     equal = false;
                    //     break;}
                    if *check_vec.get(cur_index).unwrap_or(&-1) != tmp{
                        equal = false;
                        break;
                    }
                    cur_index += 1;
                }
                tmp = 0;
            }
        }
        // equal = check_vec.len() == vec_result.len() && check_vec.iter().zip(vec_result.iter()).all(|(a, b)| a == b);
        if equal && check_vec.len() == (cur_index){
            // println!("correct:{:?}",check_vec);
            // println!("test    {:?}",vec_result);
            result += 1;
        }
    }
    result

}

// #[memoize]
fn test(input: Vec<char>, r_str: String,  total: i64, cur_total:i64) -> i64{
    let mut result = 0;
    if let Some(index) = input.iter().position(|&c| c == '?') {
        let mut tmp = input.clone();
        *tmp.get_mut(index).unwrap() = '.';
        result += test(tmp.clone(), r_str.clone(),total,cur_total);
        if cur_total < total{
            *tmp.get_mut(index).unwrap() = '#';
            result += test(tmp, r_str,total,cur_total+1);
        }
    }

    else {
        let tmp: String = input.iter().collect();
        let regex_val = Regex::new(&r_str).unwrap();
        if regex_val.is_match(&tmp){
            // println!("{:?}",input);
            result += 1
        }
    }
    result
}
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


