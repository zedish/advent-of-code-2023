use std::{io, usize};
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_hash() {
        assert_eq!(compute_hash("HASH".to_string()),52);
        assert_eq!(compute_hash("rn=1".to_string()),30);
        assert_eq!(compute_hash("cm-".to_string()),253);
        assert_eq!(compute_hash("qp=3".to_string()),97);
        assert_eq!(compute_hash("cm=2".to_string()),47);
        assert_eq!(compute_hash("qp-".to_string()),14);
        assert_eq!(compute_hash("pc=4".to_string()),180);
        assert_eq!(compute_hash("ot=9".to_string()),9);
        assert_eq!(compute_hash("ab=5".to_string()),197);
        assert_eq!(compute_hash("pc-".to_string()),48);
        assert_eq!(compute_hash("pc=6".to_string()),214);
        assert_eq!(compute_hash("ot=7".to_string()),231);
    }
    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day15_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,1320);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day15_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,145);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_solve() {
        let result = do_puzzle("day15_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,512950);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_solve() {
        let result = do_puzzle("day15_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,247153);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day15_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}


#[derive(Clone,Eq,Hash)]
#[derive(PartialEq)]
struct Opt {
    label: String,
    hash: usize,
    adding: bool,
    focal: usize,
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let mut boxes: Vec<Vec<Opt>> = vec![];
    for _ in 0..256{
        let tmp:Vec<Opt> = Vec::new();
        boxes.push(tmp);
    }

    let part1:u32 = contents.replace("\n", "").split(',').map(|val|{compute_hash_str(val)}).sum();

    
    contents.replace("\n", "").split(',').for_each(|val|{
        let tmp = create_opt(val);
        let mut found = false;
        let mut remove = false;
        let box_tmp = boxes.get_mut(tmp.hash).unwrap();
        let mut index = 0;
        for val in &mut *box_tmp{
            if val.label == tmp.label{
                found = true;
                if tmp.adding{
                   val.focal = tmp.focal; 
                } else {
                    remove = true;
                    break;
                }
            }
            index +=1;
        }
        if remove{
            box_tmp.remove(index);
        }
        if !found && tmp.focal != 0{
            boxes.get_mut(tmp.hash).unwrap().push(tmp);
        }
    });
    
    let part2 = calculate_power(boxes);
    Ok((part1 as i64,part2))
}

fn calculate_power(input: Vec<Vec<Opt>>) -> i64 {
    let mut result:i64 = 0;
    input.iter().enumerate().for_each(|(box_num, tmp_box)|{
        tmp_box.iter().enumerate().for_each(|(lens_num, lens_tmp)|{
            let tmp = (box_num+1)as i64 * (lens_num+1)as i64 * lens_tmp.focal as i64;
            result += tmp;
        });
    });

    result
}

fn create_opt(input: &str) -> Opt{
    let label_tmp = input.split(|c| c == '-' || c == '=').nth(0).unwrap().to_string();
    let focal_str = input.split(|c| c == '-' || c == '=').nth(1).unwrap_or("0");
    // println!("input:{}\tfocal:{}",input,focal_str);
    Opt{
        label : label_tmp.clone(),
        hash : compute_hash(label_tmp) as usize,
        focal :focal_str.chars().nth(0).unwrap_or('0') as usize -48,
        adding : input.contains('='),

    }
}

#[allow(dead_code)]
fn print_opt(input: Opt){
    println!("Label:{}\tHash:{}\tFocal:{}\tAdding:{}",input.label,input.hash,input.focal,input.adding);
}

fn compute_hash(input: String) -> u32 {
    let mut result = 0;
    input.chars().for_each(|c|{
       result = ( (result + c as u32) * 17) % 256
    });
    result
}
fn compute_hash_str(input: &str) -> u32 {
    compute_hash(input.to_string()) 
}


