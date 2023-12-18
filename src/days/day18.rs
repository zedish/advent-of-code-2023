use std::io;
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day18_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,62);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day18_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,952408144115);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_solve() {
        let result = do_puzzle("day18_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,48795);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_solve() {
        let result = do_puzzle("day18_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,40654918441248);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day18_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let points = calc_points(&contents, 0,0);
    let result = calc_area(points);
    
    let new_contents = contents.lines().map(|line|{
        process_line(line)
    }).collect::<Vec<String>>()
        .join("\n");
    let points2 = calc_points(&new_contents, 0,0);
    let result2 = calc_area(points2);

    Ok((result,result2))
}

fn process_line(input: &str) -> String{
    let tmp: String = input.split_whitespace().nth(2).unwrap().chars()
        .filter(|&c| c != '(' && c != ')' && c != '#')
        .collect();
    let (hex_tmp, dir_tmp) = tmp.split_at(5);

    let dir = match dir_tmp{
        "0" => 'R',
        "1" => 'D',
        "2" => 'L',
        "3" => 'U',
         _  => 'R', //this never happens
    };
    let hex = i32::from_str_radix(hex_tmp, 16).unwrap();
    let out = format!("{} {}",dir,hex);

   out 
}

fn calc_area(points: Vec<(i64,i64)>) -> i64{
    let mut prev = points[0];
    let mut result: i64 = 0;
    let mut perm = 0;
    points.iter().rev().for_each(|point|{
        result += (prev.0 * point.1) as i64;
        result -= (prev.1 * point.0) as i64;
        perm += prev.0.abs_diff(point.0) + prev.1.abs_diff(point.1);
        prev = *point;
    });
    result = (result.abs()/2 + perm as i64/2) +1;
    result
}

fn calc_points(input: &String, start_row: usize, start_col: usize) -> Vec<(i64,i64)>{
    let mut row:i64 = start_row as i64;
    let mut col:i64 = start_col as i64;
    let mut points: Vec<(i64,i64)> = Vec::new();
    input.lines().for_each(|line|{
        let mut move_dir:(i64,i64) = (0,0);
        let dir = line.split_whitespace().nth(0).unwrap();
        let dist = line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();
        match dir {
            "R" => {move_dir = (0,1)},
            "L" => {move_dir = (0,-1)},
            "D" => {move_dir = (1,0)},
            "U" => {move_dir = (-1,0)},
             _  => {},
        }
        points.push((row,col));
        for _ in 0..dist{
            row += move_dir.0;
            col += move_dir.1;
        }
    });
    points
}
