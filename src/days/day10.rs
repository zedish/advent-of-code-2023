use std::io;
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day10_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,4);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_example2() {
        let result = do_puzzle("day10_1_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,8);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day10_2_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,4);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example2() {
        let result = do_puzzle("day10_2_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,8);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example3() {
        let result = do_puzzle("day10_2_2.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,10);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}

enum ThreeBool {
    True = 1,
    False = 0,
    Unknown = 2,
}

#[derive(Clone)]
struct Location {
    north: u8,
    south: u8,
    east: u8, 
    west: u8,
    dist: u32,
    val: char,
    in_path: bool,
    dir: i8,
}

pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day10_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let mut pipes: Vec<Vec<Location>> = Vec::new();
    let mut start = (0,0);
    let mut i = 0;
    
    let line_len = contents.lines().nth(0).unwrap().len();
    let extra_line = '.'.to_string().repeat(line_len);

    let mut lines = format!("{}\n{}{}",extra_line,contents,extra_line);
    lines = lines
        .lines()
        .map(|line| format!(".{}.", line))
        .collect::<Vec<String>>()
        .join("\n");

    for line in lines.lines(){
        let mut j = 0;
        let mut vec_line: Vec<Location> = Vec::new();
        for char in line.chars(){
            let tmp = convert_char(char);
            vec_line.push(tmp);
            if char == 'S'{
                start = (i,j);
            }
            j += 1;
        }
        pipes.push(vec_line);
        i += 1;
    }


    let (mut pos1, start_loc) = find_start_locs(start, pipes.clone());
    pipes[start.0][start.1] = start_loc;
    pipes[pos1.0][pos1.1] = update_loc(pos1, start, (&pipes[pos1.0][pos1.1]).clone());
    pipes[pos1.0][pos1.1].dist = 1; 
    pipes[pos1.0][pos1.1].in_path = true;
    pipes[start.0][start.1].val = get_start_let((&pipes[start.0][start.1]).clone()); 
    let mut break_time = 0;
    let mut dist = 1;
    loop {
        if break_time > 100000{break;}
        break_time +=1;
        if pos1 == start{break;}
        dist += 1;
        
        let old_pos1 = pos1.clone();
        pos1 = find_next_pos((&pipes[pos1.0][pos1.1]).clone(), pos1);
        pipes[pos1.0][pos1.1] = update_loc(pos1, old_pos1, (&pipes[pos1.0][pos1.1]).clone());
        pipes[pos1.0][pos1.1].dist = dist;
        pipes[pos1.0][pos1.1].in_path = true;
    }
    
    let mut enclosed = 0;
    for pipe in pipes{
        let mut searching = false;
        let mut tmp_val = 0;
        for i in 0..pipe.len(){
            let loc = pipe.get(i).unwrap();
            if !searching{
                if loc.in_path{
                    searching = true;
                }
            } else {
                if loc.in_path && check_vertical(loc.val){
                    let mut num_inter = 0;
                    if tmp_val > 0 {
                        for j in i..pipe.len(){
                            if check_vertical(pipe.get(j).unwrap().val){
                                num_inter += pipe.get(j).unwrap().dir;
                            }
                        }
                        if num_inter != 0{
                            enclosed += tmp_val;
                        }
                        tmp_val = 0;
                    }
                } else if !loc.in_path{
                    tmp_val += 1;
                }
            }
        }
    }
    
    Ok(((dist/2).into(),enclosed))
}

fn get_start_let(input: Location) -> char{
    if input.north == ThreeBool::True as u8{
        if input.south == ThreeBool::True as u8{
            return '|'
        }
        if input.east == ThreeBool::True as u8{
            return 'L'
        }
        if input.west == ThreeBool::True as u8{
            return 'J'
        }
    }
    if input.south == ThreeBool::True as u8{
        if input.east == ThreeBool::True as u8{
            return 'F'
        }
        if input.west == ThreeBool::True as u8{
            return '7'
        }

    }
    if input.east == ThreeBool::True as u8{
        if input.west == ThreeBool::True as u8{
            return '-'
        }
    }
    'S'
}

fn check_vertical(c: char) -> bool {
    match c {
        '|' => return true,
        '7' => return true,
        'F' => return true,
        // 'J' => return true,
        // 'L' => return true,
         _  => return false,
        
    }

}

fn find_next_pos(input: Location, cur_pos: (usize,usize)) -> (usize,usize){
    let mut new_pos = cur_pos;
    if input.north == (ThreeBool::True as u8){
        new_pos.0 -= 1;
    }
    if input.south == (ThreeBool::True as u8){
        new_pos.0 += 1;
    }
    if input.east == (ThreeBool::True as u8){
        new_pos.1 += 1;
    }
    if input.west == (ThreeBool::True as u8){
        new_pos.1 -= 1;
    }
   new_pos 
}

fn update_loc(pos: (usize,usize), prev_pos: (usize,usize), old_loc: Location) -> Location{
    let mut new_loc = old_loc.clone();
    if (prev_pos.0 as i32 - pos.0 as i32) == -1{
        new_loc.north = ThreeBool::False as u8;
        new_loc.dir = -1;
    }
    if (prev_pos.0 as i32 - pos.0 as i32) == 1{
        new_loc.south = ThreeBool::False as u8;
        new_loc.dir = 1;
    }
    if (prev_pos.1 as i32 - pos.1 as i32) == -1{
        new_loc.west = ThreeBool::False as u8;
    }
    if (prev_pos.1 as i32 - pos.1 as i32) == 1{
        new_loc.east = ThreeBool::False as u8;
    }
    if new_loc.north == ThreeBool::True as u8{
        new_loc.dir = 1;
    }
    if new_loc.south == ThreeBool::True as u8{
        new_loc.dir = -1;
    }
    new_loc
}

fn find_start_locs(start: (usize,usize), pipes: Vec<Vec<Location>>) -> ((usize,usize),Location){
    let mut pos1 = start;
    let mut tmp = Location{
        north: ThreeBool::False as u8,
        south: ThreeBool::False as u8,
        east: ThreeBool::False as u8,
        west: ThreeBool::False as u8,
        dist: 0,
        val: 'S',
        in_path: false,
        dir: 0,
    };
    if pipes[start.0 -1][start.1].south == ThreeBool::True as u8 {
        if pos1 == start{
            pos1 = (start.0-1,start.1);
            tmp.dir = 1;
        }
        tmp.north = ThreeBool::True as u8;
    }
    if pipes[start.0 +1][start.1].north == ThreeBool::True as u8 {
        if pos1 == start{
            pos1 = (start.0+1,start.1);
            tmp.dir = -1;
        }
        tmp.south = ThreeBool::True as u8;
    }
    if pipes[start.0][start.1 + 1].west == ThreeBool::True as u8 {
        if pos1 == start{
            pos1 = (start.0,start.1 + 1);
        }
        tmp.east = ThreeBool::True as u8;
    }
    if pipes[start.0][start.1-1].east == ThreeBool::True as u8 {
        if pos1 == start{
            pos1 = (start.0,start.1-1);
        }
        tmp.west = ThreeBool::True as u8;
    }
    (pos1,tmp)
}
fn convert_char(input: char) -> Location{
    let mut tmp = Location{
        north: ThreeBool::Unknown as u8,
        south: ThreeBool::Unknown as u8,
        east: ThreeBool::Unknown as u8,
        west: ThreeBool::Unknown as u8,
        dist: 0,
        val: input,
        in_path: false,
        dir: 0,
    };
    match input {
        '-' => {
                tmp.north = ThreeBool::False as u8; 
                tmp.south = ThreeBool::False as u8; 
                tmp.east = ThreeBool::True as u8; 
                tmp.west = ThreeBool::True as u8; 
                }
        '|' => {
                tmp.north = ThreeBool::True as u8; 
                tmp.south = ThreeBool::True as u8; 
                tmp.east = ThreeBool::False as u8; 
                tmp.west = ThreeBool::False as u8; 
                }
        'F' => {
                tmp.north = ThreeBool::False as u8; 
                tmp.south = ThreeBool::True as u8; 
                tmp.east = ThreeBool::True as u8; 
                tmp.west = ThreeBool::False as u8; 
                }
        'J' => {
                tmp.north = ThreeBool::True as u8; 
                tmp.south = ThreeBool::False as u8; 
                tmp.east = ThreeBool::False as u8; 
                tmp.west = ThreeBool::True as u8; 
                }
        'L' => {
                tmp.north = ThreeBool::True as u8; 
                tmp.south = ThreeBool::False as u8; 
                tmp.east = ThreeBool::True as u8; 
                tmp.west = ThreeBool::False as u8; 
                }
        '7' => {
                tmp.north = ThreeBool::False as u8; 
                tmp.south = ThreeBool::True as u8; 
                tmp.east = ThreeBool::False as u8; 
                tmp.west = ThreeBool::True as u8; 
                }
        '.' => {
                tmp.north = ThreeBool::False as u8; 
                tmp.south = ThreeBool::False as u8; 
                tmp.east = ThreeBool::False as u8; 
                tmp.west = ThreeBool::False as u8; 
                }
        's' => {
                tmp.north = ThreeBool::Unknown as u8; 
                tmp.south = ThreeBool::Unknown as u8; 
                tmp.east = ThreeBool::Unknown as u8; 
                tmp.west = ThreeBool::Unknown as u8; 
                }
         _  => {
                tmp.north = ThreeBool::Unknown as u8; 
                tmp.south = ThreeBool::Unknown as u8; 
                tmp.east = ThreeBool::Unknown as u8; 
                tmp.west = ThreeBool::Unknown as u8; 
                } 
        
    }
    tmp
}
