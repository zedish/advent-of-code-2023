use std::io;
use crate::utils;

pub fn solve() -> (i32,i32) {
    let result = do_puzzle("day2_1.txt",vec![12,13,14]);
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    (0,0)
}

fn do_puzzle(input: &str, valid_colors: Vec<i32>) -> Result<(i32,i32), io::Error>{
    let contents = utils::read_file(input)?;
    let mut game_num = 1;
    let mut result = 0;
    let mut power = 0;
    for line in contents.lines(){
        let tmp_return = parse_line(line);
        let mut parsed_line: Vec<Vec<i32>> = Vec::new();
        let mut min_needed = vec![0,0,0];
        match tmp_return {
        Ok(value) => parsed_line = value,
        Err(error) => println!("Error: {}", error),
        }
        let mut game_bad = false;
        for game_part in parsed_line {
            let mut i = 0;
            for color in &valid_colors{
                if *color < game_part[i]{
                    game_bad = true;
                }
                if min_needed[i] < game_part[i]{
                    min_needed[i] = game_part[i];
                }

                i += 1;
            }
        
        }
        if !game_bad {
            result += game_num;
        }
        game_num += 1;

        power += min_needed[0]*min_needed[1]*min_needed[2];
    }
    Ok((result,power))
}

fn parse_line(input: &str) -> Result<Vec<Vec<i32>>, &'static str> {
    let colors: [&str; 3] = ["red", "green", "blue"];
    let mut final_vec: Vec<Vec<i32>> = Vec::new(); 
    
    let games: Vec<&str> = input.splitn(2, ':').collect();
    let result;
    if games.len() == 2 {
        result = games[1];
    } else {
        println!("Error delimiter not found for line:{}", input);
        return Err("Error delimiter not found");
    }
    let entries: Vec<&str> = result.split(';').collect();
    for entry in entries {
        let mut color_list = vec![0,0,0];
        let values: Vec<&str> = entry.split(',').collect();
        for value in values {
            let mut i = 0;
            for string in &colors {
                if let Some(_index) = value.find(string){
                    let mut new_value = value.replace(string,"");
                    new_value = new_value.replace(" ","");
                    color_list[i] = utils::convert_to_int(new_value.as_str());
                }
                i += 1;
            }
        }
        final_vec.push(color_list);
    }
    Ok(final_vec)
}
