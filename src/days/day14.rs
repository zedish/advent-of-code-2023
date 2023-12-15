use std::io;
use crate::utils;
use nalgebra::DMatrix;
extern crate nalgebra as na;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day14_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,136);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day14_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,64);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_solve() {
        let result = do_puzzle("day14_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,113424);
                          assert_eq!(value.1,96003);
                        }
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}

#[derive(Clone, Copy,Eq,Hash)]
#[derive(PartialEq)]
enum Rocks {
    Moveable,
    Nonmoveable,
    Empty,
}

fn rocks_to_numeric(rock: Rocks) -> f64 {
    match rock {
        Rocks::Moveable => 3.0,
        Rocks::Nonmoveable => 2.0,
        Rocks::Empty => 1.0,
    }
}

pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day14_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let matrix: Vec<Vec<Rocks>> = contents.lines().map(|line| {
        line.chars().map(|c|{
            match c {
                'O' => Rocks::Moveable,
                '#' => Rocks::Nonmoveable,
                 _  => Rocks::Empty,
            }
        }).collect()
    }).collect();

    let mut e_vec: Vec<na::Matrix<na::Complex<f64>, na::Dyn, na::Const<1>, na::VecStorage<na::Complex<f64>, na::Dyn, na::Const<1>>>> = Vec::new();

    let mut part1_matrix = matrix.clone();
    let mut part2_matrix = matrix.clone();

    rotate_matrix_ccw(&mut part1_matrix);
    move_left(&mut part1_matrix);
    rotate_matrix_cw(&mut part1_matrix);
    let part1 = calculate(part1_matrix);

    rotate_matrix_ccw(&mut part2_matrix);
    e_vec.push(compute_eigen(part2_matrix.clone()));
    let mut extra_needed = 0;
    let mut loop_len = 0;
    part2_matrix = do_cycle(part2_matrix);
    for i  in 1..=1000000000_u64{
        part2_matrix = do_cycle(part2_matrix);        
        let tmp_e = compute_eigen(part2_matrix.clone());
        let check = check_if_loop(tmp_e.clone(), e_vec.clone());
        
        if check != -1{
            loop_len = i as i32 -check;
            
            //not entirely sure why i is +2 here but it works
            extra_needed = (1000000000 - (i + 2)) %loop_len as u64;
            break;
        }
        e_vec.push(tmp_e);
    }
    for _ in 0..=extra_needed + loop_len as u64{
        part2_matrix = do_cycle(part2_matrix.clone());
    }
    rotate_matrix_cw(&mut part2_matrix);
    let part2 = calculate(part2_matrix);
    Ok((part1,part2))
}

fn check_if_loop(cur: na::Matrix<na::Complex<f64>, na::Dyn, na::Const<1>, na::VecStorage<na::Complex<f64>, na::Dyn, na::Const<1>>>, prev: Vec<na::Matrix<na::Complex<f64>, na::Dyn, na::Const<1>, na::VecStorage<na::Complex<f64>, na::Dyn, na::Const<1>>>>) -> i32{
    for (i,val) in prev.iter().enumerate(){
        if cur == *val{
            return i as i32
        }
    }

   return -1; 
}

fn compute_eigen(input: Vec<Vec<Rocks>>) -> na::Matrix<na::Complex<f64>, na::Dyn, na::Const<1>, na::VecStorage<na::Complex<f64>, na::Dyn, na::Const<1>>>{
    let numeric_matrix: Vec<Vec<f64>> = input.iter()
        .map(|row| row.iter().map(|&rock| rocks_to_numeric(rock)).collect())
        .collect();
    let nrows = numeric_matrix.len();
    let ncols = numeric_matrix[0].len();
    let mut d_matrix = DMatrix::zeros(nrows, ncols);

    for i in 0..nrows {
        for j in 0..ncols {
            d_matrix[(i, j)] = numeric_matrix[i][j];
        }
    }
    d_matrix.complex_eigenvalues()
} 

fn do_cycle(input: Vec<Vec<Rocks>>) -> Vec<Vec<Rocks>>{
    let mut output = input.clone();
    //north
    move_left(&mut output);
    rotate_matrix_cw(&mut output);
    //west
    move_left(&mut output);
    rotate_matrix_cw(&mut output);
    //south
    move_left(&mut output);
    rotate_matrix_cw(&mut output);
    //east
    move_left(&mut output);
    rotate_matrix_cw(&mut output);
    output
}

fn calculate(input: Vec<Vec<Rocks>>) -> i64 {
    let num_lines = input.len();
    let mut result: i64 = 0;
    
    for i in 0..num_lines{
        let mut tmp = 0;
        for c in &input[i]{
            if *c == Rocks::Moveable {
                tmp += 1;
            }
        }
        result += (tmp * (num_lines-i)) as i64;
    }
    result
}

fn rotate_matrix_cw(matrix: &mut Vec<Vec<Rocks>>) {
    let n = matrix.len();

    for x in 0..n / 2 {
        for y in x..n - x - 1 {
            let temp = matrix[x][y];
            matrix[x][y] = matrix[n - 1 - y][x];
            matrix[n - 1 - y][x] = matrix[n - 1 - x][n - 1 - y];
            matrix[n - 1 - x][n - 1 - y] = matrix[y][n - 1 - x];
            matrix[y][n - 1 - x] = temp;
        }
    }
}

fn rotate_matrix_ccw(matrix: &mut Vec<Vec<Rocks>>) {
    let n = matrix.len();

    for x in 0..n / 2 {
        for y in x..n - x - 1 {
            let temp = matrix[x][y];
            matrix[x][y] = matrix[y][n - 1 - x];
            matrix[y][n - 1 - x] = matrix[n - 1 - x][n - 1 - y];
            matrix[n - 1 - x][n - 1 - y] = matrix[n - 1 - y][x];
            matrix[n - 1 - y][x] = temp;
        }
    }
}

fn move_left(input: &mut Vec<Vec<Rocks>>) {
    let _ = input.clone().iter().enumerate().for_each(|(index_i, line)|{
        let tmp = move_line(line.to_vec());
        input[index_i] = tmp;
    });
}

fn move_line(input: Vec<Rocks>) -> Vec<Rocks> {
    let mut output = input.clone(); 
    let mut valid = find_next_valid(input.clone(), 0);
    let mut index = valid + 1;
    loop {
        match output[index] {
            Rocks::Moveable => {
                output[valid] = Rocks::Moveable;
                output[index] = Rocks::Empty;
                valid = find_next_valid(output.clone(), valid);
                index += 1;
            },
            Rocks::Nonmoveable => {
                valid = find_next_valid(output.clone(), index);
                index = valid + 1;
            },
            Rocks::Empty => { index += 1},
        }
        if index > input.len()-1{break;}
    }
    output
}

fn find_next_valid(input: Vec<Rocks>, start: usize) -> usize{
    for i in start..input.len(){
        if input[i] == Rocks::Empty {
            return i;
        }
    }
    100
}

#[allow(dead_code)]
fn print_matrix(input: Vec<Vec<Rocks>>){
    input.iter().for_each(|line|{
        print_line(line.to_vec());
    });
}

#[allow(dead_code)]
fn print_line(input: Vec<Rocks>){
    input.iter().for_each(|rock| {
        match rock {
            Rocks::Moveable => print!("O"),
            Rocks::Nonmoveable => print!("#"),
            Rocks::Empty => print!("."),
        };
    });
    println!();
}
