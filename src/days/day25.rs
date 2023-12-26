use std::collections::{HashSet, HashMap};
use std::io;
use crate::utils;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::Result;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day25_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,54);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_solve() {
        let result = do_puzzle("day25_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,544523);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day25_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

type RwxResult<T> = Result<Option<(usize, Vec<T>)>, Box<dyn Error>>;

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let lines:Vec<Vec<String>> = contents.lines().map(|line| line.split(&[':',' '])
                       .filter(|val| !val.is_empty())
                       .map(|line| line.to_string()).collect()).collect();

    let mut graph = UnGraph::new_undirected();

    let vertices = lines.iter().flatten().map(|s| s.as_str()).collect::<HashSet<_>>();
    let nodes = vertices.iter().map(|&s| (s, graph.add_node(s))).collect::<HashMap<_, _>>();

    for val in &lines {
        let v1 = val.get(0).unwrap().as_str();
        for v2 in val[1..].iter().map(|s| s.as_str()) {
            graph.add_edge(nodes[v1], nodes[v2], 1);
        }
    }
    let min_cut: RwxResult<_> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let mut part1 = 0;
    if let Ok(Some((_, cut))) = &min_cut {
        part1 = (vertices.len() - cut.len()) * cut.len();
    }

    Ok((part1 as i64,0))
}




