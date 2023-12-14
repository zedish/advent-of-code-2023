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
    // for line in contents.lines(){
    //     println!("{line}");
    // }
    let input: Vec<&str> = contents.split("\n\n").collect();
    
    for val in input{
        let mut input_vec: Vec<Vec<char>> = val.split("\n").map(|line| line.chars().collect()).collect();
        input_vec.retain(|line| !line.is_empty());
        // println!("{}",val);
        let horz_vec = convert_bin(input_vec.clone());
        let hor_line = find_sym(horz_vec.clone());
        // println!("base");
        // for v in horz_vec{
        //     print!("{v}");
        //     println!("");
        // }
        // for val in &input_vec{
        //     for c in val {
        //         print!("{}",c);
        //     }
        //     println!(":{}",val.len());
        // }
        // println!("input  {}:{}",input_vec.len(),input_vec[0].len());
        // let horz_vec = transpose(input_vec);
        let vert_vec = convert_bin(transpose(input_vec));
        let vert_line = find_sym(vert_vec.clone());

        // println!("result {}:{}", vert_line, hor_line);
        result += vert_line as i64;
        result += hor_line as i64 * 100;
        // for v in vert_vec{
        //     print!("{v}");
        //     println!("");
        // }
        // for v in horz_vec{
        //     for j in v{
        //         print!("{j}");
        //     }
        //     println!("");
        //
        // }
        // println!("between");
    }
    Ok((result,0))
}
fn find_sym(input: Vec<u64>) ->usize{
    let mut index1 = 0;
    let mut found = false;
    // result = 0;
    for index2 in 1..input.len(){
        // println!("{}:{}",input[index1],input[index2]);
        
        if input[index1] == input[index2]{
            // println!("found");
            found = true;
            if index1 == 0 {
                // println!("found thing");
                break;
            }
            let mut inner1 = index1 - 1;
            for inner2 in index2+1..input.len(){
                // println!("inner1:{}",inner1);
                // println!("{}:{}",input[inner1],inner1);
                // println!("{}:{}",input[inner2],inner2);
                if input[inner1] != input[inner2]{
                    // println!("failed");
                    found = false;
                    break;
                }
                // println!("inner1:{}",inner1);
                if inner1 == 0{
                    break;
                }
                inner1 -= 1;
                // println!("inner1:{}",inner1);
            }
            if found{
                // println!("actually found {}:{}",index1+1,index2+1);
                return index1+1;
            } else {
                // println!("not found yet");
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
fn convert_bin(input: Vec<Vec<char>>) -> Vec<u64>{
    let result: Vec<u64> = input.iter().map(|val|{
        // u64::from_str_radix((val.iter().map(|c| if c == &'#' { '1' } else { '0' }).collect::<String>()),2).unwrap_or(0)
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
    
    // println!("Matrix  {}:{}", num_rows,num_cols);
    let mut transposed: Vec<Vec<char>> = vec![vec![' '; num_rows]; num_cols];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            transposed[j][i] = ch;
        }
    }
    // for i in 0..num_rows {
    //     for j in 0..num_cols {
    //         transposed[j][i] = matrix[i][j];
    //     }
    // }

    transposed
}
// fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
//     if matrix.is_empty() {
//         return vec![];
//     }
//
//     let rows = matrix.len();
//     let cols = matrix[0].len();
//
//     let mut result = vec![vec!['.'; rows]; cols];
//
//     for i in 0..rows {
//         for j in 0..cols {
//             println!("{}:{}",i,j);
//             result[j][i] = matrix[i][j];
//         }
//     }
//
//     result
// }



