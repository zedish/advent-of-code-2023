use std::collections::HashMap;
use std::io;
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_find_numbers() {
        let val = find_nums("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(val.0,8);
        let val = find_nums("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        assert_eq!(val.0,2);
        let val = find_nums("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        assert_eq!(val.0,2);
        let val = find_nums("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
        assert_eq!(val.0,1);
        let val = find_nums("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
        assert_eq!(val.0,0);
        let val = find_nums("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(val.0,0);
    }

    #[test]
    fn test_part1_example() {
        let result = do_puzzle("day4_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,13);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    
    #[test]
    fn test_part2_example() {
        let result = do_puzzle("day4_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,30);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}

pub fn solve() -> (i32,i32) {
    let result = do_puzzle("day4_1.txt");
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i32,i32), io::Error>{
    let contents = utils::read_file(input)?;
    let mut num_cards: Vec<i32> = vec![1; contents.lines().count()];
    let mut result1 = 0;
    let mut result2 = 0;

    let mut i = 0;
    for line in contents.lines(){
        let result = find_nums(line);
        result1 += result.0;
        if let Some(mult) = num_cards.get(i).cloned(){
            for val in 1..=result.1{
                if let Some(value) = num_cards.get_mut((val as usize)+i) {
                    *value += mult; // Increment the value at the specified index
                }
            }
            result2 += mult;
        }
        
        i += 1;
    }
    Ok((result1,result2))
}

fn find_nums(input: &str) -> (i32,i32) {
    let mut numbers: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;
    let mut result2 = 0;
    let parts: Vec<&str> = input.split(": ").collect();
        if let Some(second_half) = parts.get(1) {
            let vals: Vec<&str> = second_half.split(" | ").collect();
            if let Some(first_nums) = vals.get(0){
                let nums: Vec<&str> = first_nums.split_whitespace().collect();
                for entry in nums{
                    numbers.entry(utils::convert_to_int(entry)).or_insert(1);
                }
            }
            if let Some(second_nums) = vals.get(1){
                let nums: Vec<&str> = second_nums.split_whitespace().collect();
                for entry in nums{
                    if let Some(value) = numbers.get_mut(&utils::convert_to_int(entry)){
                        *value += 1;
                        if result == 0{ 
                            result = 1;
                        } else {
                            result = result * 2;    
                        }
                        result2 += 1;
                    }
                }
            }
        }
    (result,result2)
}
