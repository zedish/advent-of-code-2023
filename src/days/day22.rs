use std::io;
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
    }
}


pub fn solve() -> (i32,i32) {
    let result = do_puzzle("day4_1_0.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i32,i32), io::Error>{
    let contents = utils::read_file(input)?;

    for line in contents.lines(){
        println!("{line}");
    }

    Ok((0,0))
}




