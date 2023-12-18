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
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day18_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

#[derive(Clone)]
struct Cell {
    wall: bool
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    
    // for line in contents.lines(){
    //     println!("{line}");
    // }
    let ((max_rows,max_cols),(start_row,start_col)) = get_size(&contents);
    println!("Rows:{}    Cols:{}", max_rows,max_cols);


    let mut matrix: Vec<Vec<Cell>> = vec![vec![Cell{
        wall : false
    };max_cols]; max_rows];

    // let points = calc_wall(contents, start_row,start_col, &mut matrix);
    let points = calc_points(contents, 0,0);
    // print_map(&matrix);
    // println!("{:?}",points);
    let result = calc_area_2(points);
    // print_rot_map(&matrix); 
    Ok((result,0))
}

fn calc_area(points: Vec<(usize,usize)>) -> i64{
    let mut prev = points[0];
    let mut result: i64 = 0;
    let mut perm = 0;
    points.iter().rev().for_each(|point|{
        result += (prev.0 * point.1) as i64;
        result -= (prev.1 * point.0) as i64;
        perm += prev.0.abs_diff(point.0) + prev.1.abs_diff(point.1);
        prev = *point;
    });

    result = (result + perm as i64)/2 +1;
    result
}
fn calc_area_2(points: Vec<(i32,i32)>) -> i64{
    let mut prev = points[0];
    let mut result: i64 = 0;
    let mut perm = 0;
    points.iter().rev().for_each(|point|{
        result += (prev.0 * point.1) as i64;
        result -= (prev.1 * point.0) as i64;
        perm += prev.0.abs_diff(point.0) + prev.1.abs_diff(point.1);
        prev = *point;
    });

    result = (result + perm as i64)/2 +1;
    result
}

fn print_map(matrix : &Vec<Vec<Cell>>){
    matrix.iter().for_each(|row|{
        row.iter().for_each(|val|{
            if val.wall{
                print!("#");
            } else {
                print!(".");
            }
        });
        println!("");
    });
}

fn print_rot_map(matrix : &Vec<Vec<Cell>>){
    for i in 0..matrix[0].len(){
        for j in 0..matrix.len(){
            let val = &matrix[j][i];
                if val.wall{
                    print!("#");
                } else {
                    print!(".");
                }
        }
        println!("");
    }
}
fn calc_points(input: String, start_row: usize, start_col: usize) -> Vec<(i32,i32)>{
    let mut row:i32 = start_row as i32;
    let mut col:i32 = start_col as i32;
    let mut points: Vec<(i32,i32)> = Vec::new();
    input.lines().for_each(|line|{
        let mut move_dir:(i8,i8) = (0,0);
        let dir = line.split_whitespace().nth(0).unwrap();
        let dist = line.split_whitespace().nth(1).unwrap().parse::<i8>().unwrap();
        match dir {
            "R" => {move_dir = (0,1)},
            "L" => {move_dir = (0,-1)},
            "D" => {move_dir = (1,0)},
            "U" => {move_dir = (-1,0)},
             _  => {},
        }
        points.push((row,col));
        for _ in 0..dist{
            row += (move_dir.0)as i32;
            col += (move_dir.1)as i32;
            // println!("row:{}   col:{}",row,col);
        }
    });
    points
}

fn calc_wall(input: String, start_row: usize, start_col: usize, matrix: &mut Vec<Vec<Cell>>) -> Vec<(usize,usize)>{
    let mut row:i32 = start_row as i32;
    let mut col:i32 = start_col as i32;
    let mut points: Vec<(usize,usize)> = Vec::new();
    input.lines().for_each(|line|{
        let mut move_dir:(i8,i8) = (0,0);
        let dir = line.split_whitespace().nth(0).unwrap();
        let dist = line.split_whitespace().nth(1).unwrap().parse::<i8>().unwrap();
        match dir {
            "R" => {move_dir = (0,1)},
            "L" => {move_dir = (0,-1)},
            "D" => {move_dir = (1,0)},
            "U" => {move_dir = (-1,0)},
             _  => {},
        }
        points.push((row as usize+1,col as usize+1));
        for _ in 0..dist{
            row += (move_dir.0)as i32;
            col += (move_dir.1)as i32;
            // println!("row:{}   col:{}",row,col);
            matrix[row as usize][col as usize].wall=true;
        }
    });
    points
}

fn calc_line(line: String, row: i32, col: i32, matrix: &mut Vec<Vec<Cell>>, points: &mut Vec<(usize,usize)>) {

}

fn get_size(input: &String) -> ((usize,usize),(usize,usize)){
    let mut cols:i32 = 1;
    let mut max_cols = 0;
    let mut min_cols = 0;
    input.lines().filter(|line|line.contains("R") || line.contains("L")).for_each(|line|{
        if line.contains("R"){
            cols += line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap()
        } else {
            cols -= line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap()
        }
        if cols > max_cols{
            max_cols = cols;
        } 
        if cols < min_cols{
            min_cols = cols;
        } 
    });
    let mut rows:i32 = 1;
    let mut max_rows = 0;
    let mut min_rows = 0;
    input.lines().filter(|line|line.contains("D") || line.contains("U")).for_each(|line|{
        if line.contains("D"){
            rows += line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap()
        } else {
            rows -= line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap()
        }
        if rows > max_rows{
            max_rows = rows;
        } 
        if rows < min_rows{
            min_rows = rows;
        } 
    });

    if min_cols < 0 {
        min_cols -= 1;
    }
    if min_rows < 0 {
        min_rows -= 1;
    }
    // println!("max_row:{}  min_row:{}   max_col:{}  min_col:{}", max_rows, min_rows, max_cols, min_cols);
    (((max_rows - min_rows) as usize,(max_cols - min_cols) as usize),
    ((0 - min_rows) as usize,(0 - min_cols) as usize))
}



