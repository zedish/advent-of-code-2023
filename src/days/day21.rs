use std::io;
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day21_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,16);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_solve() {
        let result = do_puzzle("day21_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,3795);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_solve() {
        let result = do_puzzle("day21_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,630129824772393);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day21_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let mut grid:Vec<Vec<char>> = contents.lines().map(|line|{line.chars().collect()}).collect();
    let mut steps_1 = 64;
    if input.contains("21_1_0"){
        steps_1 = 6;
    } 
    let mut visted: Vec<Vec<i32>> = grid.iter().map(|row|{
        row.iter().map(|val|{
            let mut ret_val = -2;
            if *val == '#'{ 
                ret_val = -1 
            } 
            if *val == 'S' {
                ret_val = 0
            } 
            ret_val
        }).collect()
    }).collect();
    
    let mut step_cnt = 1;
    let mut should_continue = true;
    while should_continue{
        should_continue = find_and_step(&mut grid, &mut visted, &mut step_cnt);
    }
    let part1 = visted.iter().flat_map(|row| row.iter()).filter(|val| **val > -1 && **val <= steps_1 && *val %2 == steps_1%2).count();
  
    let half = ((visted.len() -1)/2) as i32;
    let even_corners = visted.iter().flat_map(|row| row.iter()).filter(|val| **val > -1 && **val %2 == 0 && **val > half).count();
    let odd_corners = visted.iter().flat_map(|row| row.iter()).filter(|val| **val > -1 && **val %2 == 1 && **val > half).count();
   
    let even_full = visted.iter().flat_map(|row|row.iter()).filter(|val| **val > -1 && **val %2 == 0).count();
    let odd_full = visted.iter().flat_map(|row|row.iter()).filter(|val| **val > -1 && **val %2 == 1).count();

    let num_steps_2 = 26501365;
    let n = ((num_steps_2 - (visted.len()/2)) / visted.len()) as usize;
    let part2 = ((n+1)*(n+1)) * odd_full + (n*n) * even_full - (n+1) * odd_corners + (n * even_corners);

    Ok((part1 as i64,part2 as i64))
}

fn find_and_step(input: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<i32>>, step_cnt: &mut i32) -> bool{
    let mut return_val = false;
    input.clone().iter().enumerate().for_each(|(row_index,row)|{
        row.iter().enumerate().for_each(|(col_index,val)|{
            if *val == 'O' || *val == 'S' {
                if step_pos(input, (row_index,col_index), visited, step_cnt) {
                    return_val = true;
                }
            } 
        })
    });
    *step_cnt += 1;
    return_val
}

fn step_pos(input: &mut Vec<Vec<char>>, pos: (usize,usize), visited: &mut Vec<Vec<i32>>, step_cnt: &mut i32) -> bool{
    let positions = vec![(pos.0+1,pos.1),(pos.0.wrapping_sub(1),pos.1),
                        (pos.0,pos.1+1),(pos.0,pos.1.wrapping_sub(1))];
    let new_pos:Vec<(usize,usize)> = positions.into_iter().filter_map(|(row,col)|{
        if row < input.len() && col < input[0].len(){
            if visited[row][col] == -2 {
                Some((row,col))
            } else {
                None
            }
        } else {
            None
        }
    }).collect();
    let mut return_val = false;
    new_pos.iter().for_each(|(row,col)|{
        return_val = true;
        input[*row][*col] = 'O';
        visited[*row][*col] = step_cnt.clone();
    });
    input[pos.0][pos.1] = '.';
    return_val
}


