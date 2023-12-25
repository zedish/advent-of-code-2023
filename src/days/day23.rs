use std::io;
use crate::utils;
use std::collections::{HashMap, HashSet, VecDeque};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day23_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,94);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day23_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,154);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}

pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day23_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

#[derive(Clone)]
struct Node {
    neighbors: Vec<usize>,
    neighbors2: Vec<usize>,
    val: char,
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let num_cols = contents.lines().next().unwrap().chars().count();
    let mut graph: HashMap<usize,Node> = contents.lines().enumerate().flat_map(|(row,line)|{
        line.chars().enumerate().map(move |(col,c)|{
        let tmp = Node {
                neighbors: Vec::new(),
                neighbors2: Vec::new(),
                val: c,
            };
        (col + (row * num_cols) as usize,tmp)
        })
    }).collect();

    gen_neighbors(&mut graph, num_cols);

    let mut start_node = 0;
    contents.lines().next().unwrap().chars().enumerate().for_each(|(index,val)|{
        if val == '.' {
            start_node = index + (0 * num_cols);
        }
    });
    let mut end_node = 0;
    contents.lines().last().unwrap().chars().enumerate().for_each(|(index,val)|{
        if val == '.' {
            end_node = index + ((contents.lines().count()-1)*num_cols);
        }
    });

    let mut dec_nodes = find_decisions(&graph);
    dec_nodes.push(start_node);
    dec_nodes.push(end_node);

    let mut shrunk_graph1: HashMap<usize,Vec<(usize,usize)>> = HashMap::new();
    let mut shrunk_graph2: HashMap<usize,Vec<(usize,usize)>> = HashMap::new();
    dec_nodes.iter().for_each(|node|{
        shrunk_graph1.insert(*node, find_next_dec(&graph, *node,1));
        shrunk_graph2.insert(*node, find_next_dec(&graph, *node,2));
    });

    let part1 = weight_bfs_longest_path(&shrunk_graph1, start_node, end_node, &mut Vec::new(), 0);
    let part2 = weight_bfs_longest_path(&shrunk_graph2, start_node, end_node, &mut Vec::new(), 0);
    Ok((part1 as i64, part2 as i64))
}

fn find_next_dec(graph: &HashMap<usize,Node>, start_node: usize, part: usize) -> Vec<(usize,usize)>{
    let mut prev = 0;
    let mut cur_node = start_node;
    let mut steps = 0;
    let mut result: Vec<(usize,usize)> = Vec::new();
    let start_nodes:Vec<usize> = graph.get(&cur_node).unwrap().neighbors.iter().filter_map(|node|{
        if *node != prev {
            Some(*node)
        } else {
            None
        }
    }).collect();
    start_nodes.iter().for_each(|tmp_node|{
        prev = start_node;
        cur_node = *tmp_node;
        steps = 1;
        loop {
            let mut tmp_neighbors = &graph.get(&cur_node).unwrap().neighbors;
            if part == 2 {
                tmp_neighbors = &graph.get(&cur_node).unwrap().neighbors2;
            }
            let next_nodes:Vec<usize> = tmp_neighbors.iter().filter_map(|node|{
                if *node != prev {
                    Some(*node)
                } else {
                    None
                }
            }).collect();
            if next_nodes.len() == 1 {
                steps += 1;
                prev = cur_node;
                cur_node = *next_nodes.get(0).unwrap();
            } 
            else {
                result.push((cur_node,steps));
                break;
            }
        }
    });
    result 
}

fn find_decisions(graph: &HashMap<usize,Node>) -> Vec<usize>{
    let mut result: Vec<usize> = Vec::new();
    graph.iter().for_each(|(key,val)|{
        if val.neighbors.len()>2{
            result.push(*key);
        }
    });
    result
}

fn weight_bfs_longest_path(graph: &HashMap<usize,Vec<(usize,usize)>>, start_node: usize, end_node: usize, visited: &mut Vec<usize>, depth_start: usize) -> usize {
    let mut queue = VecDeque::new();
    visited.push(start_node);
    queue.push_back((start_node, depth_start)); // (node, depth)

    let mut longest_path:usize = 0;

    while let Some((node, depth)) = queue.pop_front() {
        if node == end_node {
            longest_path = longest_path.max(depth);
            break;
        }
        if let Some(tmp_node) = graph.get(&node) {
            let mut first = true;
            let tmp_visited = visited.clone();
            for neighbor in tmp_node {
                if !visited.contains(&neighbor.0) {
                    if first {
                        first = false;
                        visited.push(neighbor.0);
                        queue.push_back((neighbor.0,depth + neighbor.1));
                    }
                    else {
                        longest_path = longest_path.max(weight_bfs_longest_path(&graph, neighbor.0, end_node, &mut tmp_visited.clone(), depth+neighbor.1));
                    }
                }
            }
        }
    }

    longest_path
}

fn gen_neighbors(input: &mut HashMap<usize,Node>, num_cols: usize){
    let max_val = input.len();
    let tmp_map = input.clone();
    input.iter_mut().for_each(|(key,node)|{
        node.neighbors = get_valid_pos(&tmp_map, &key, num_cols, max_val,1);
        node.neighbors2 = get_valid_pos(&tmp_map, &key, num_cols, max_val,2);
    });
}

fn get_valid_pos(input: &HashMap<usize,Node>,pos: &usize, num_cols: usize, max_val: usize, part: usize) -> Vec<usize> {
    let cur_x = pos/num_cols;
    let cur_y = pos%num_cols;
    let cur_char = input.get(&pos).unwrap().val;
    let mut positions:Vec<(i32,i32)> = vec![(0,1),(0,-1),(1,0),(-1,0)];
    if part == 1 {
        match cur_char {
            '>' => { positions = vec![(0,1)]},
            '<' => { positions = vec![(0,-1)]},
            '^' => { positions = vec![(-1,0)]},
            'v' => { positions = vec![(1,0)]},
            '#' => { return vec![]},
            _   => {},
        };
    } else if cur_char == '#' {
        return vec![];
    }

    let filter_pos:Vec<usize> = positions.iter().filter_map(|(row,col)|{
        let new_x = cur_x as i32 + row;
        let new_y = cur_y as i32 + col;
        let index = new_y + (new_x*num_cols as i32);
        if new_x < 0 || new_y < 0 {
            None
        }
        else if new_y as usize >= num_cols || index as usize >= max_val{
            None
        }
        else if input.get(&(index as usize)).unwrap().val == '#' {
            None
        }
        else {
            Some(index as usize)
        }
    }).collect();
   filter_pos 
}

#[allow(dead_code)]
fn print_graph(input: &HashMap<usize,Node>, num_cols: usize, visited: HashSet<usize>){
    for i in 0..input.len(){
        if i%num_cols == 0 {
            println!("");
        }
        if visited.contains(&i) {
            print!("O");
        }
        else {
            print!("{}",input.get(&i).unwrap().val);
        }
    }
    println!("");
}
