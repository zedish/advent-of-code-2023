use std::io;
use crate::utils;
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        let result = do_puzzle("day5_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,35);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_examples() {
        let result = do_puzzle("day5_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,46);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day5_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let progress = "/-\\-"; 

    let seeds = contents.lines().nth(0)
                        .unwrap()
                        .split(':')
                        .nth(1)
                        .unwrap()
                        .split_whitespace()
                        .map(|x| x.parse::<i64>()
                        .unwrap());
    let mut seed_vec: Vec<i64> = Vec::new();
    for seed in seeds.clone(){
        seed_vec.push(seed);
    }
    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();    

    let mut skip2 = 3;
    let num_lines = contents.lines().count();
    while num_lines > skip2{
        let tmp1 = get_next_block(contents.clone(), skip2);
        maps.push(tmp1.clone());
        skip2 += tmp1.len() + 2;
    }
 
    let mut lowest: i64 = i64::MAX;
    for seed in seeds.clone(){
        let test = get_dist(seed,maps.clone());
        if test < lowest{
            lowest = test;
        }
    }
    let mut part2: i64 = i64::MAX;
    let max_seed = get_max_seed(seed_vec.clone());
    let mut thing = 0;
    for i in 0..=max_seed{
        let tmp_seed = get_seed(i, maps.clone());
        if check_valid_seed(tmp_seed, seed_vec.clone()){
            part2 = i;
            break;
        }
        thing += 1;
        if thing > 50000{
            thing = 0;
            print!("\r{}\tSeed:{}",progress.chars().nth((i%4)as usize).unwrap(),i);
            std::io::stdout().flush().unwrap();
        }
    }
    print!("\r");
    std::io::stdout().flush().unwrap();
    let val1:i32 = lowest.try_into().unwrap();
    let val2:i32 = part2.try_into().unwrap(); 
    Ok((val1 as i64,val2 as i64))
}
fn get_max_seed(seeds: Vec<i64>) -> i64{
    let mut prev = -1;
    let mut max = 0;
    for seed in seeds{
        if prev == -1{
            prev = seed;
        } else {
            let val = prev + seed;
            if val > max{
                max = val
            }
            prev = -1
        }
    }
    max
}

fn check_valid_seed(input: i64, seeds: Vec<i64>) -> bool{
    let mut prev = -1;
    let mut valid = false;
    for seed in seeds{
        if prev == -1{
            prev = seed;
        } else {
            if input >= prev && input < prev + seed{
                valid = true;
            }
            prev = -1
        }
    }

    valid
}

fn get_seed(seed: i64, maps: Vec<Vec<Vec<i64>>>) -> i64{
    let mut tmp = seed;
    for map in maps.iter().rev(){
        for val in map{
            let dest = val.get(1).unwrap_or(&0);
            let source = val.get(0).unwrap_or(&0);
            let range = val.get(2).unwrap_or(&0);
            let dif = dest - source;

            let bottom = source;
            let top = source + range -1;

            if tmp >= *bottom && tmp <= top{
                tmp += dif;
                break;
            }   
        }
    }
    tmp
}
fn get_dist(seed: i64, maps: Vec<Vec<Vec<i64>>>) -> i64{
    let mut tmp = seed;
    for map in &maps{
        for val in map{
            let source = val.get(1).unwrap_or(&0);
            let dest = val.get(0).unwrap_or(&0);
            let range = val.get(2).unwrap_or(&0);
            let dif = dest - source;

            let bottom = source;
            let top = source + range -1;

            if tmp >= *bottom && tmp <= top{
                tmp += dif;
                break;
            }
         }
    }
    tmp
}

fn get_next_block(lines: String, skip: usize) -> Vec<Vec<i64>>{
    let result: Vec<Vec<i64>> = lines.lines()
                            .skip(skip)
                            .take_while(|&line| !line.trim().is_empty())
                            .map(|line| line.split_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect())
                            .collect();
    result    
}
