use std::io;
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day9_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,114);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day9_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,2);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day9_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let lines:Vec<Vec<i32>> = contents.lines()
                                .map(|line|
                                line.split_whitespace()
                                .map(|x|x.parse::<i32>()
                                .unwrap()).collect())
                                .collect();

    let mut results:(i32,i32) = (0,0);
    let mut vals:(i32,i32);
    for line in lines{
        vals = get_diffs(line);
        results.0 += vals.0;
        results.1 += vals.1;
    }

    Ok((results.0 as i64, results.1 as i64))
}

fn get_diffs(input: Vec<i32>) -> (i32,i32){
    let mut diffs: Vec<i32> = Vec::new();
    let mut nonzero = false;
    let mut last_val = 0;
    let first_val = input.get(0).unwrap();
    for i in 0..input.len(){
        if let Some(next) = input.get(i+1){
            last_val = *next;
            let tmp_dif = next - input.get(i).unwrap();
            diffs.push(tmp_dif);
            
            if tmp_dif != 0 {
                nonzero = true;
            }
        }
    }
    let mut val = 0;
    let mut val2 = 0;
    if nonzero{
        (val,val2) = get_diffs(diffs);
    }
    (last_val + val, first_val - val2)
}
