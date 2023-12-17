use std::io;
use crate::utils;
use pathfinding::prelude::dijkstra;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day17_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,102);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day17_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,94);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example2() {
        let result = do_puzzle("day17_1_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,71);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_solve() {
        let result = do_puzzle("day17_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,674);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_solve() {
        let result = do_puzzle("day17_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,773);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day17_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

const ROWS: usize = 150;
const COLS: usize = 150;

#[derive(PartialEq,Clone,Hash,Eq)]
struct History {
    prev_pos: Pos,
    dir: (i32,i32),
    num: usize,
    goal: Pos,
}

fn get_valid_pos_1(input: Pos, history: History) -> Vec<(Pos,History)>{
    let Pos(x,y) = input;
    let positions = vec![Pos(x+1,y), Pos(x-1,y),
         Pos(x,y+1), Pos(x,y-1)];
    let mut new_positions: Vec<(Pos,History)> = Vec::new();
    for i in 0..positions.len(){
        let next_pos = positions.get(i).unwrap();
        if history.prev_pos == *next_pos {
            continue;
            //skip the loop so we don't go backward
        }
        let mut new_history = history.clone();

        let dir = ((next_pos.0 - x),(next_pos.1 - y));
        if dir == new_history.dir{
            new_history.num += 1;
        } else {
            new_history.num = 1;
        }
        if new_history.num < 4 {
            new_history.prev_pos = input.clone();
            new_history.dir = dir;
            new_positions.push((next_pos.clone(),new_history));
        }
    }
    new_positions
}

fn get_valid_pos_2(input: Pos, history: History) -> Vec<(Pos,History)>{
    let Pos(x,y) = input;
    let positions = vec![Pos(x+1,y), Pos(x-1,y),
         Pos(x,y+1), Pos(x,y-1)];
    let mut new_positions: Vec<(Pos,History)> = Vec::new();
    for i in 0..positions.len(){
        let next_pos = positions.get(i).unwrap();
        if history.prev_pos == *next_pos {
            continue;
            //skip the loop so we don't go backward
        }
        if history.goal == *next_pos && history.num < 3{
            continue;
        }
        let mut new_history = history.clone();

        let dir = ((next_pos.0 - x),(next_pos.1 - y));

        if input == Pos(0,0) {  //on the start point go either way
            if dir == (0,1) || dir == (1,0){
                new_history.num = 2;
                new_history.prev_pos = input.clone();
                new_history.dir = dir;
                new_positions.push((next_pos.clone(),new_history.clone()));
            }
                continue;
        }
        if dir == new_history.dir{
            new_history.num += 1;
            if new_history.num < 11 {
                new_history.prev_pos = input.clone();
                new_history.dir = dir;
                new_positions.push((next_pos.clone(),new_history.clone()));
            }  
        } else {
            if  new_history.num > 3 { 
                new_history.num = 1;
                new_history.prev_pos = input.clone();
                new_history.dir = dir;
                new_positions.push((next_pos.clone(),new_history.clone()));
            }
        }
    }
    new_positions
}

#[derive(PartialEq,Clone,Hash,Eq)]
struct Pos(i32,i32);
impl Pos {
    fn successors_1(&self, weights: &[[usize; ROWS];COLS], history: History) -> Vec<((Pos, History), usize)> {
        get_valid_pos_1(self.clone(), history).into_iter().map(|p| {
            let weight = get_weight(&p.0,weights);
            (p, weight)
        }).collect::<Vec<((Pos,History),usize)>>()
      }
    fn successors_2(&self, weights: &[[usize; ROWS];COLS], history: History) -> Vec<((Pos, History), usize)> {
        get_valid_pos_2(self.clone(), history).into_iter().map(|p| {
            let weight = get_weight(&p.0,weights);
            (p, weight)
        }).collect::<Vec<((Pos,History),usize)>>()
      }
}
fn get_weight(input: &Pos, weights: &[[usize; ROWS];COLS] ) -> usize{
    *weights.get(input.0 as usize).and_then(|row| row.get(input.1 as usize)).unwrap_or(&2000)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let mut weights = [[2000 as usize; ROWS]; COLS]; // static size based on puzzle input
    contents.lines().enumerate().for_each(|(index_row,line)|{
        line.chars().enumerate().for_each(|(index_col, val)|{
            weights[index_row][index_col] = val.to_digit(10).unwrap_or(2000) as usize;
        })
    });

    let goal: Pos =  Pos((contents.lines().count() - 1).try_into().unwrap(),
                        (contents.lines().nth(0).unwrap().len() - 1).try_into().unwrap());
    let history = History {
        dir: (0,1),
        prev_pos: Pos(0,0),
        num: 1,
        goal: goal.clone(),
    };
    let result = dijkstra(&(Pos(0, 0), history.clone()), |(p , history)| p.successors_1(&weights, history.clone()), |(p,_history)| *p == goal).expect("couldn't find path");

    let result2 = dijkstra(&(Pos(0, 0), history.clone()), |(p , history)| p.successors_2(&weights, history.clone()), |(p,_history)| *p == goal).expect("couldn't find path");

    Ok((result.1 as i64,result2.1 as i64))
}
