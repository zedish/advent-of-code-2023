use std::io;
use crate::utils;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::collections::VecDeque;

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

// #[derive(Clone,Copy)]
// struct Pos{
//     pos: (i32,i32),
//     weight: i32,
// 
#[derive(PartialEq,Clone,Hash,Eq)]
struct History {
    prev_pos: Pos,
    dir: (i32,i32),
    num: usize,
}

#[derive(PartialEq,Clone,Hash,Eq)]
struct Pos(i32,i32);
impl Pos {
    // fn default()->Self{
    //     Self {
    //         pos: (0,0),
    //         weight: 0,
    //     }
    // }
    fn successors(&self, weights: &[[usize; ROWS];COLS], prev_locations: VecDeque<Pos>) -> Vec<((Pos, VecDeque<Pos>), usize)> {
        let &Pos(x, y) = self;
        let positions = vec![Pos(x+1,y), Pos(x-1,y),
             Pos(x,y+1), Pos(x,y-1)];
        let mut new_positions: Vec<(Pos,VecDeque<Pos>)> = Vec::new();
        let mut to_be_removed: Vec<usize> = Vec::new();
        for i in 0..positions.len(){
            let next_pos = positions.get(i).unwrap();
            if prev_locations.len() > 2 && prev_locations[1] == *next_pos {
                continue;
                //skip the loop so we don't go backward
            }

            let mut new_deque = prev_locations.clone();
            new_deque.push_front(next_pos.clone());

            if new_deque.len() == 5 {
                let dir = ((new_deque[1].0 - new_deque[0].0),(new_deque[1].1 - new_deque[0].1));
                let a = ((new_deque[2].0 - new_deque[1].0),(new_deque[2].1 - new_deque[1].1));
                let b = ((new_deque[3].0 - new_deque[2].0),(new_deque[3].1 - new_deque[2].1));
                let c = ((new_deque[4].0 - new_deque[3].0),(new_deque[4].1 - new_deque[3].1));
                let three_forward_check = [
                    a,
                    b,
                    c,
                ].iter().all(|a_dir| a_dir == &dir);
                    
                if three_forward_check {
                    to_be_removed.push(i);
                } else {
                    new_deque.pop_back();
                    new_positions.push((next_pos.clone(),new_deque));
                }
            } else {
                new_positions.push((next_pos.clone(),new_deque));
            }
        }
        new_positions.into_iter().map(|p| {
            let weight = get_weight(&p.0,weights);
            (p, weight)
        }).collect::<Vec<((Pos,VecDeque<Pos>),usize)>>()
      }
    fn successors_2(&self, weights: &[[usize; ROWS];COLS], history: History) -> Vec<((Pos, History), usize)> {
        let &Pos(x, y) = self;
        let positions = vec![Pos(x+1,y), Pos(x-1,y),
             Pos(x,y+1), Pos(x,y-1)];
        let mut new_positions: Vec<(Pos,History)> = Vec::new();
        let mut to_be_removed: Vec<usize> = Vec::new();
        for i in 0..positions.len(){
            let next_pos = positions.get(i).unwrap();
            // println!("new:{},{}\tprev:{},{}",next_pos.0,next_pos.1,history.prev_pos.0,history.prev_pos.1);
            if history.prev_pos == *next_pos {
                // println!("skip going backward");
                continue;
                //skip the loop so we don't go backward
            }
            let mut new_history = history.clone();

            let dir = ((x - next_pos.0),(y - next_pos.1));
            if dir == new_history.dir{
                new_history.num += 1;
            } else {
                new_history.num = 1;
            }
            if new_history.num < 4 {
                // new_history.prev_pos = self.clone();
                new_history.prev_pos = self.clone();
                new_history.dir = dir;
                new_positions.push((next_pos.clone(),new_history));
            }
            // let mut new_deque = prev_locations.clone();
            // new_deque.push_front(next_pos.clone());
            //
            // if new_deque.len() == 5 {
            //     let dir = ((new_deque[1].0 - new_deque[0].0),(new_deque[1].1 - new_deque[0].1));
            //     let a = ((new_deque[2].0 - new_deque[1].0),(new_deque[2].1 - new_deque[1].1));
            //     let b = ((new_deque[3].0 - new_deque[2].0),(new_deque[3].1 - new_deque[2].1));
            //     let c = ((new_deque[4].0 - new_deque[3].0),(new_deque[4].1 - new_deque[3].1));
            //     let three_forward_check = [
            //         a,
            //         b,
            //         c,
            //     ].iter().all(|a_dir| a_dir == &dir);
            //         
            //     if three_forward_check {
            //         to_be_removed.push(i);
            //     } else {
            //         new_deque.pop_back();
            //         new_positions.push((next_pos.clone(),new_deque));
            //     }
            // } else {
            //     new_positions.push((next_pos.clone(),new_deque));
            // }
        }
        new_positions.into_iter().map(|p| {
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

    // for line in contents.lines(){
    //     println!("{line}");
    // }
    let mut weights = [[2000 as usize; ROWS]; COLS]; // static size based on puzzle input
    contents.lines().enumerate().for_each(|(index_row,line)|{
        line.chars().enumerate().for_each(|(index_col, val)|{
            weights[index_row][index_col] = val.to_digit(10).unwrap_or(2000) as usize;
        })
    });

    let GOAL: Pos =  Pos((contents.lines().count() - 1).try_into().unwrap(),
                        (contents.lines().nth(0).unwrap().len() - 1).try_into().unwrap());
    let mut prev_locations:  VecDeque<Pos> = VecDeque::new();
    prev_locations.push_front(Pos(0,0));
    // let result = dijkstra(&(Pos(0, 0), prev_locations.clone()), |(p , prev_locations)| p.successors(&weights, prev_locations.clone()), |(p,_prev_locations)| *p == GOAL).expect("couldn't find path");
    //
    let mut history = History {
        dir: (0,1),
        prev_pos: Pos(0,0),
        num: 1,
    };
    let result = dijkstra(&(Pos(0, 0), history.clone()), |(p , history)| p.successors_2(&weights, history.clone()), |(p,_history)| *p == GOAL).expect("couldn't find path");
    // for val in prev_locations{
    //     println!("{},{}",val.0,val.1);
    // }
    // println!("Result:{}",result.1);
   
    let mut path: Vec<Pos> = Vec::new();
    for val in result.0{
        // println!("{},{}", val.0.0,val.0.1);
        path.push(val.0);
    }
    contents.lines().enumerate().for_each(|(index_row, line)|{
        line.chars().enumerate().for_each(|(index_col, val)|{
            if check_in_path(Pos(index_row as i32,index_col as i32), path.clone()){
                print!("#");
            } else {
                // print!("{}",val);
                print!(".");
            }
        });
        println!("");
    });

    Ok((result.1 as i64,0))
}

fn check_in_path(input: Pos, path: Vec<Pos>) -> bool{
    let mut return_val = false;
    path.iter().for_each(|check|{
        if check.0 == input.0 && check.1 == input.1{
            return_val = true;
        } 
    });
    return_val
}


