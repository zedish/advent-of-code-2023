use std::{io, usize};
use crate::utils;
use gcd::Gcd;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day8_1_0.txt",false);
        match result{
            Ok(value) => {assert_eq!(value.0,2);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_example2() {
        let result = do_puzzle("day8_1_1.txt",false);
        match result{
            Ok(value) => {assert_eq!(value.0,6);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day8_2_0.txt",true);
        match result{
            Ok(value) => {assert_eq!(value.1,6);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day8_1.txt",false); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str, skip1: bool)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let lines: Vec<&str> = contents.split("\n\n").collect();
    let mut array: [(usize,usize,bool); 26*26*26] = [(0,0,false); (26*26*26)];

    let mut part2_indices: Vec<usize> = Vec::new();
    let dir = lines.get(0).unwrap();
    for line in lines.get(1).unwrap().lines(){
        let index_string = line.split("=").nth(0).unwrap().replace(" ", "");
        let dests = line.split("=").nth(1).unwrap().replace(" ","").replace("(","").replace(")","");
        let left = dests.split(",").nth(0).unwrap();
        let right = dests.split(",").nth(1).unwrap();
        let index = calc_index(&index_string);
        let left_index = calc_index(left);
        let right_index = calc_index(right);
        array[index] = (left_index,right_index,index_string.chars().last().unwrap()=='Z');
        if index_string.chars().last().unwrap() == 'A'{
            part2_indices.push(index);
        }
    }
    let mut cur_index = 0;
    let mut steps = 0;
    if !skip1{
        loop{
            for c in dir.chars(){
                if cur_index == 17575{
                    break;
                }
                steps += 1;
                if c == 'R'{
                    cur_index = array[cur_index].1;
                } else {
                    cur_index = array[cur_index].0;
                }
            }
            if cur_index == 17575{
                break;
            }
        }
    }
    let mut steps2:u64 = 0;
    
    let mut part2_hops: Vec<u64> = Vec::new();
    for index in &part2_indices{
        cur_index = *index;
        loop{
            for c in dir.chars(){
                // if array[cur_index].2{
                //     println!("in direction loop");
                //     part2_hops.push(steps2);
                //     break;
                // }
                steps2 += 1;
                if c == 'R'{
                    cur_index = array[cur_index].1;
                } else {
                    cur_index = array[cur_index].0;
                }
            }
            if array[cur_index].2{
                part2_hops.push(steps2);
                break;
            }
        }
        steps2 = 0;
    }

    let part2_result = lcm_of_multiple(&part2_hops);
    Ok((steps as i64,part2_result as i64))
}

// fn invert_index(input: usize) -> String{
//     let val1 = ((input%17576/676) as u8 + 65) as char;
//     let val2 = ((input%676/26) as u8 + 65) as char;
//     let val3 = ((input%26) as u8 + 65) as char;
//     format!("{}{}{}",val1,val2,val3)
// }
fn calc_index(input: &str) -> usize{
    ((input.chars().nth(2).unwrap() as u32)%65 + 
    (input.chars().nth(1).unwrap() as u32)%65 * 26 + 
    (input.chars().nth(0).unwrap() as u32)%65 * 676) as usize 
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / a.gcd(b)) * b
}
fn lcm_of_multiple(numbers: &[u64]) -> u64 {
    if numbers.len() < 2 {
        panic!("At least two numbers are required to find LCM");
    }

    let mut result = numbers[0];
    for &num in &numbers[1..] {
        result = lcm(result, num);
    }
    result
}

