use std::io;
use std::collections::HashMap;
use crate::utils;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_examples() {
        let result = do_puzzle("day3_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,467835);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_examples() {
        let result = do_puzzle("day3_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,4361);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}

pub fn solve() -> io::Result<()> {
    let result = do_puzzle("day3_1.txt");

    match result{
        Ok(value) => {println!("Part 1 Result:{}\tPart 2 Result:{}", value.0, value.1);}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    Ok(())
}

fn do_puzzle(input: &str)-> Result<(i32,i32), io::Error>{
    println!("Solving puzzle for file {}", input);

    let contents = utils::read_file(input)?;
    let extra_line = '.'.to_string().repeat(10);

    let mut lines = format!("{}\n{}{}",extra_line,contents,extra_line);
    lines = lines
        .lines()
        .map(|line| format!(".{}.", line))
        .collect::<Vec<String>>()
        .join("\n");

    let mut char_2d_array: Vec<Vec<char>> = Vec::new();
    for line in lines.lines(){
        let chars: Vec<char> = line.chars().collect();
        char_2d_array.push(chars);
    }
    let result = find_num_in_line(&char_2d_array);
    Ok(result)
}

fn find_num_in_line(input: &Vec<Vec<char>>) -> (i32,i32) {
    let mut result = 0;
    let mut numbers: Vec<(i32, bool, Vec<i32>)> = Vec::new();
    let mut gears_vals: HashMap<i32,Vec<i32>> = HashMap::new();
    let mut gears: Vec<i32> = Vec::new();
    for j in 0..input.len() {
        let row = &input[j];
        let mut num = String::new();
        let mut is_valid = false;

        for i in 0..row.len(){
            let c = row[i];
            let c2 = row.get(i+1).copied().unwrap_or('.');
            if c.is_digit(10){
                num = format!("{}{}",num,c);
                for k in (-1i32..=1).rev(){
                    if is_valid{
                        break;
                    }
                    let k_val = (i as i32 + k) as usize;
                    for l in (-1i32..=1).rev(){
                        let l_val = (j as i32 + l) as usize;
                        let loc = input.get(l_val)
                                        .and_then(|row| row.get(k_val))
                                        .copied()
                                        .unwrap_or('.');
                        if loc == '*' {
                            gears.push(utils::convert_to_int(format!("{}{}",l_val,k_val).as_str()));
                        }
                        if loc != '.' && !loc.is_digit(10){
                            is_valid = true;
                            break;
                        }
                    }
                }
                if !c2.is_digit(10){
                    numbers.push((utils::convert_to_int(&num),is_valid,gears.clone()));
                    if is_valid{
                        result += utils::convert_to_int(num.as_str());
                        is_valid = false;
                    }
                    num.clear();
                    gears.clear();
                }
            }
        }
    }
    
    for tuple in &numbers {
        for value in &tuple.2{
            gears_vals.entry(*value).or_insert(Vec::new()).push(tuple.0);
        }
    }
    let mut result2 = 0;
    for tuple in &gears_vals{
        if tuple.1.len() == 2 {
            if let Some(value1) = tuple.1.get(0) {
                if let Some(value2) = tuple.1.get(1) {
                    result2 += value1 * value2;
                } else {
                    println!("Index out of bounds");
                }
            } else {
                println!("Index out of bounds");
            }
        }
    }
    (result,result2)
}
