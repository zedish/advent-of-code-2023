use std::io;
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        let result = do_puzzle("day6_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,288);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_examples() {
        let result = do_puzzle("day6_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,71503);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i32,i32) {
    let result = do_puzzle("day6_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i32,i32), io::Error>{
    let contents = utils::read_file(input)?;
    let mut inputs: Vec<Vec<i64>> = Vec::new(); 
    let mut inputs2: Vec<i64> = Vec::new(); 
    
    for line in contents.lines() {
        let vals = line.split(':').nth(1).unwrap_or("0 0 0");
        let vals2 = vals.replace(" ","");
        let nums: Vec<i64> = vals.split_whitespace()
                        .map(|x| x.parse::<i64>()
                        .unwrap_or(0))
                        .collect();
        inputs.push(nums);
        inputs2.push(vals2.parse::<i64>().unwrap_or(0));
    }

    let vec1 = inputs.get(0).unwrap();
    let vec2 = inputs.get(1).unwrap();
    
    let mut part1 = 1;
    for i in 0..vec1.len(){
        part1 = part1 * get_margin(*vec1.get(i).unwrap_or(&0), 
                                   *vec2.get(i).unwrap_or(&0));
    }
    let part2 = get_margin(*inputs2.get(0).unwrap_or(&0), 
                           *inputs2.get(1).unwrap_or(&0)); 

    Ok((part1 as i32,part2 as i32))
}

fn get_margin(b_in: i64, c_in: i64) -> i64{
    let (val1,val2) = quad(1, -1*b_in, c_in);
        
    let lower = if val1 < val2 { 
        (val1+1.0).floor() as i64
    } else { 
        (val2+1.0).floor() as i64
    };

    let higher = if val1 > val2 { 
        (val1-1.0).ceil() as i64
    } else { 
        (val2-1.0).ceil() as i64
    };

    higher-lower + 1
}

fn quad(a: i64, b: i64, c :i64) -> (f64,f64){
    let sqrt = (((b*b)-(4*a*c)) as f64).sqrt();
    let top1 = -1.0*b as f64 + sqrt;
    let top2 = -1.0*b as f64 - sqrt;
    let bot = 2.0*a as f64;
    
    (top1/bot,top2/bot)
}



