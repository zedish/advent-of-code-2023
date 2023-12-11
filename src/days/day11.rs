use std::{io, collections::HashSet};
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day11_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,374);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_answer() {
        let result = do_puzzle("day11_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,9693756);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_answer() {
        let result = do_puzzle("day11_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,717878258016);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day11_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;

    let parsed2: Vec<Vec<char>> = contents
        .lines()
        .map(|line|{
            line.chars().collect()
        }).collect();
    
    let rows: HashSet<i64> = parsed2
        .iter()
        .enumerate()
        .filter(|(_, row)| !row.contains(&'#'))
        .map(|(index, _)| index as i64)
        .collect();
    
    let cols: HashSet<i64> = transpose(&parsed2)
        .iter()
        .enumerate()
        .filter(|(_, row)| !row.contains(&'#'))
        .map(|(index, _)| index as i64)
        .collect();
    
    let mut points2: Vec<(i64,i64)> = Vec::new();
    for i in 0..parsed2.len(){
        for j in 0..parsed2[0].len(){
            if parsed2[i][j] == '#'{
                points2.push((i as i64,j as i64));
            }
        }
    }
    
    let mut result2: i64 = 0;
    let mut result: i64 = 0;
    for (i, point) in points2.iter().enumerate(){
        for point2 in &points2[i + 1..]{
            let tmp_dist = (point.0 - point2.0).abs() +
                            (point.1 - point2.1).abs(); 
            result += tmp_dist;
            result2 += tmp_dist;
            
            let max0 = &point.0.max(point2.0);
            let min0 = &point.0.min(point2.0);
            let max1 = &point.1.max(point2.1);
            let min1 = &point.1.min(point2.1);
            for row in &rows{
                if (min0..max0).contains(&row){
                    result += 1; 
                    result2 += 999999; 
                }
            }
            for col in &cols{
                if (min1..max1).contains(&col){
                    result += 1; 
                    result2 += 999999; 
                }
            }
        } 
    }
    Ok((result.into(),result2.into()))
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() {
        return vec![];
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = vec![vec!['.'; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = matrix[i][j];
        }
    }

    result
}


