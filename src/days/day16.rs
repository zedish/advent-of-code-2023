use std::io;
use crate::utils;
use std::thread;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day16_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,46);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day16_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,51);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_solve() {
        let result = do_puzzle("day16_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,7798);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}

#[derive(Clone, Eq,Hash)]
#[derive(PartialEq, Copy)]
enum Mirror {
    MLeft,
    MRight,
    SHor,
    SVert,
    Empty,
}

#[derive(Clone, Eq,Hash)]
#[derive(PartialEq,Copy)]
struct Cell {
    c_type: Mirror,
    energized: bool,
    approach: [[bool; 3]; 3],
}

#[derive(Clone, Eq,Hash)]
#[derive(PartialEq)]
struct Light {
    location: (i8,i8),
    direction: (i8,i8),
}

fn create_cell(input: char) -> Cell {
    Cell {
        energized: false,
        c_type: {
            match input {
                '.'  => Mirror::Empty,
                '/'  => Mirror::MLeft,
                '\\' => Mirror::MRight,
                '|'  => Mirror::SVert,
                '-'  => Mirror::SHor,
                 _   => Mirror::Empty,
            }
        },
        approach: [[false; 3]; 3],
    }
}

pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day16_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let matrix: Vec<Vec<Cell>> = contents.lines().map(|line| {
        line.chars().map(|c|{
            create_cell(c)
        }).collect()
    }).collect();

    let start_light = Light {
                        location: (0,0),
                        direction: (0,1),
                    };
    let mut matrix0 = matrix.clone();
    follow_light(&mut matrix0, start_light.clone());
    let part1 = count_energized(matrix0.clone());

    let part2 = do_part2_thread(matrix, start_light);

    Ok((part1,part2))
}

fn do_thing(mut matrix: &mut Vec<Vec<Cell>>, start_light: Light)->i64{
    follow_light(&mut matrix, start_light);
    count_energized(matrix.to_vec())
}

fn do_part2_thread(matrix: Vec<Vec<Cell>>, start_light: Light) -> i64{
    let mut part2 = 0;
    let mut vals = vec![];
    let mut starts: Vec<Light> = Vec::new();
    for i in 0..matrix.len(){
        let mut start_light1 = start_light.clone(); 
        let mut start_light2 = start_light.clone(); 
        start_light1.location = (i as i8,0);
        start_light1.direction = (0,1);
        starts.push(start_light1);
        
        start_light2.location = (i as i8, (matrix.len()-1) as i8);
        start_light2.direction = (0,-1);
        starts.push(start_light2);
    }
    for i in 0..matrix[0].len(){
        let mut start_light1 = start_light.clone(); 
        let mut start_light2 = start_light.clone(); 
        
        start_light1.location = (0,i as i8);
        start_light1.direction = (1,0);
        starts.push(start_light1);
        
        start_light2.location = ((matrix.len()-1) as i8,i as i8);
        start_light2.direction = (-1,0);
        starts.push(start_light2);
    }

    let num_threads = 8;
    let partitions: Vec<Vec<Light>> = starts
        .chunks(starts.len() / num_threads) // Split into chunks of equal length
        .map(|chunk| chunk.to_vec())
        .collect();

    partitions.iter().for_each(|part|{
        let tmp_part = part.clone();
        let tmp_matrix = matrix.clone();
        let val = thread::spawn(move || process_list(tmp_matrix,tmp_part));
        vals.push(val);
    });


    for val in vals{
        let tmp = val.join().unwrap();
        if tmp > part2{
            part2 = tmp;
        }
    }

    part2
}

fn process_list(matrix: Vec<Vec<Cell>>, inputs: Vec<Light>) -> i64 {
    let mut intern_matrix = matrix.clone();
    let mut result = 0;
    inputs.iter().for_each(|val|{
        set_matrix(&matrix, &mut intern_matrix);
        let tmp = do_thing(&mut intern_matrix, val.clone());
        if tmp > result{
            result = tmp;
        }

    });
    result
}

fn set_matrix(input:&Vec<Vec<Cell>>, output: &mut Vec<Vec<Cell>>){
    for (inner_vec1, inner_vec2) in output.iter_mut().zip(input.iter()) {
            for (elem1, &elem2) in inner_vec1.iter_mut().zip(inner_vec2.iter()) {
                *elem1 = elem2;
        }
    }
}

fn count_energized(matrix: Vec<Vec<Cell>>) -> i64{
    let mut result = 0;
    matrix.iter().for_each(|val|{
        val.iter().for_each(|cell|{
            if cell.energized{
                result += 1;
            }
        });
    });
    result
}

fn follow_light(matrix: &mut Vec<Vec<Cell>>, mut start_pos: Light){
    let mut valid = true;
    while valid {
        let  approach_vec = matrix[start_pos.location.0 as usize][start_pos.location.1 as usize]
                .approach[(start_pos.direction.0 + 1) as usize][(start_pos.direction.1 + 1) as usize];
        if approach_vec{
            break;
        }
        matrix[start_pos.location.0 as usize][start_pos.location.1 as usize].energized = true;
        matrix[start_pos.location.0 as usize][start_pos.location.1 as usize]
                .approach[(start_pos.direction.0 + 1) as usize][(start_pos.direction.1 + 1) as usize] = true;
        (start_pos,valid) = move_one(matrix, start_pos);
    }
}

fn start_new_light(matrix: &mut Vec<Vec<Cell>>, prev_pos: (i8,i8), new_dir: (i8,i8)){
    let pos = Light{
        location: ((prev_pos.0 + new_dir.0),(prev_pos.1 + new_dir.1)),
        direction: new_dir,
    }; 
    if pos.location.0 < 0 || pos.location.1 < 0 {
        return;
    }
    if pos.location.0 >= matrix.len() as i8 || pos.location.1 >= matrix[0].len() as i8{
        return;
    }
    follow_light(matrix, pos);
}

fn move_one(matrix: &mut Vec<Vec<Cell>>, start_pos: Light) -> (Light,bool){
    let cur_mirror = &matrix[start_pos.location.0 as usize][start_pos.location.1 as usize].c_type;
    let mut new_dir:(i8,i8) = (0,0);
    match *cur_mirror{
            Mirror::Empty => {new_dir = start_pos.direction},
            Mirror::MLeft => {      // "/"
                    match start_pos.direction{
                        (0,1) => {new_dir = (-1,0)},
                        (1,0) => {new_dir = (0,-1)},
                        (0,-1) => {new_dir = (1,0)},
                        (-1,0) => {new_dir = (0,1)},
                          _   => {},
                    }
            },
            Mirror::MRight => {     // "\"
                    match start_pos.direction{
                        (0,1) => {new_dir = (1,0)},
                        (1,0) => {new_dir = (0,1)},
                        (0,-1) => {new_dir = (-1,0)},
                        (-1,0) => {new_dir = (0,-1)},
                          _   => {},
                    }
            },
            Mirror::SHor => {     // "-"
                    match start_pos.direction{
                        (0,1) => {new_dir = (0,1)},
                        (1,0) => {new_dir = (0,1);
                                  //need to do other dir
                                    start_new_light(matrix, (start_pos.location.0,start_pos.location.1), (0,-1));
                                },
                        (0,-1) => {new_dir = (0,-1)},
                        (-1,0) => {new_dir = (0,1);
                                  //need to do other dir
                                    start_new_light(matrix, (start_pos.location.0,start_pos.location.1), (0,-1));
                                },
                          _   => {},
                    }
            },
            Mirror::SVert => {     // "|"
                    match start_pos.direction{
                        (0,1) => {new_dir = (1,0);
                                  //need to do other dir
                                    start_new_light(matrix, (start_pos.location.0,start_pos.location.1), (-1,0));
                                },
                        (1,0) => {new_dir = (1,0)},
                        (0,-1) => {new_dir = (1,0);
                                  //need to do other dir
                                    start_new_light(matrix, (start_pos.location.0,start_pos.location.1), (-1,0));
                                },
                        (-1,0) => {new_dir = (-1,0)},
                          _   => {},
                    }
            },
    }
    let new_pos = Light{
        location: ((start_pos.location.0 + new_dir.0),(start_pos.location.1 + new_dir.1)),
        direction: new_dir,
    }; 

    if new_pos.location.0 < 0 || new_pos.location.1 < 0 {
        return (new_pos, false);
    }
    if new_pos.location.0 >= matrix.len() as i8 || new_pos.location.1 >= matrix[0].len() as i8{
        return (new_pos,false);
    }
    (new_pos,true)
}


#[allow(dead_code)]
fn print_matrix(input: Vec<Vec<Cell>>){
    input.iter().for_each(|line|{
        print_line(line.to_vec());
    });
}

#[allow(dead_code)]
fn print_line(input: Vec<Cell>){
    input.iter().for_each(|cell| {
        match cell.c_type {
            Mirror::Empty => print!("."),
            Mirror::MLeft => print!("/"),
            Mirror::MRight => print!("\\"),
            Mirror::SHor => print!("-"),
            Mirror::SVert => print!("|"),
        };
    });
    println!();
}

#[allow(dead_code)]
fn print_mirror(input: Mirror){
    match input {
        Mirror::Empty => print!("."),
        Mirror::MLeft => print!("/"),
        Mirror::MRight => print!("\\"),
        Mirror::SHor => print!("-"),
        Mirror::SVert => print!("|"),
    };
}

#[allow(dead_code)]
fn print_matrix_energized(input: Vec<Vec<Cell>>){
    input.iter().for_each(|line|{
        print_line_energized(line.to_vec());
    });
}

#[allow(dead_code)]
fn print_line_energized(input: Vec<Cell>){
    input.iter().for_each(|cell| {
        if cell.energized{
            print!("#");
        } else {
            match cell.c_type {
                Mirror::Empty => print!("."),
                Mirror::MLeft => print!("/"),
                Mirror::MRight => print!("\\"),
                Mirror::SHor => print!("-"),
                Mirror::SVert => print!("|"),
            };
        }
    });
    println!();
}
