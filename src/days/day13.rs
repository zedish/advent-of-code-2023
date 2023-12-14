use std::{io, u64};
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day13_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,405);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day13_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,400);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_solve() {
        let result = do_puzzle("day13_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,33780);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_solve() {
        let result = do_puzzle("day13_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,23479);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day13_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;

    let mut result: i64 = 0;
    let mut result2: i64 = 0;
    let input: Vec<&str> = contents.split("\n\n").collect();
    
    for val in input{
        let mut input_vec: Vec<Vec<char>> = val.split("\n").map(|line| line.chars().collect()).collect();
        input_vec.retain(|line| !line.is_empty());

        let horz_vec = convert_bin(input_vec.clone());
        let hor_line = find_sym(horz_vec.clone(),true,-1);
        let hor_line2 = find_sym(horz_vec.clone(),false, hor_line as i32);
        
        let vert_vec = convert_bin(transpose(input_vec));
        let vert_line = find_sym(vert_vec.clone(),true,-1);
        let vert_line2 = find_sym(vert_vec.clone(),false,vert_line as i32);

        result += vert_line as i64;
        result += hor_line as i64 * 100;
        result2 += vert_line2 as i64;
        result2 += hor_line2 as i64 * 100;
    }
    Ok((result,result2))
}
fn find_sym(input: Vec<u64>,mode:bool,prev1:i32) ->usize{
    let mut index1 = 0;
    let mut found = false;
    for index2 in 1..input.len(){
        if prev1 == index1 as i32 +1{
            index1 +=1;
            continue;
        }
        let mut used_fuzzy = mode;
        if diff_bits(input[index1], input[index2],&mut used_fuzzy) {
            found = true;
            if index1 == 0 {
                break;
            }
            let mut inner1 = index1 - 1;
            for inner2 in index2+1..input.len(){
                if !diff_bits(input[inner1], input[inner2],&mut used_fuzzy) {
                    found = false;
                    break;
                }
                if inner1 == 0{
                    break;
                }
                inner1 -= 1;
            }
        }
        if found{
            return index1+1;
        }
        index1 += 1;
    }
    if found{
        return index1+1;
    }
    0
}
fn diff_bits(a: u64, b: u64, fuzzy: &mut bool) -> bool {
    let mut count = 0;
    let mut xor_result = a ^ b;

    while xor_result > 0 {
        if xor_result & 1 == 1 {
            count += 1;
        }
        xor_result >>= 1;
    }
    if count == 0{
        return true
    }
    if count == 1 && !*fuzzy{
        *fuzzy = true;
        return true;
    }
    return false;
}
fn convert_bin(input: Vec<Vec<char>>) -> Vec<u64>{
    let result: Vec<u64> = input.iter().map(|val|{
        let bin_str: String = val.iter().map(|c| if c == &'#' { '1' } else { '0' }).collect();
        u64::from_str_radix(&bin_str, 2).unwrap_or(0)
    }).collect();
    result
}
fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() {
        return Vec::new(); // Return an empty vector if input is empty
    }

    let num_rows = matrix.len();
    let num_cols = matrix[0].len();
    
    let mut transposed: Vec<Vec<char>> = vec![vec![' '; num_rows]; num_cols];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            transposed[j][i] = ch;
        }
    }
    transposed
}
