use std::io;
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        let result = do_puzzle("day5_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,35);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_examples() {
        let result = do_puzzle("day5_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,46);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_solve() {
        let result = do_puzzle("day5_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,600279879);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_solve() {
        let result = do_puzzle("day5_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,20191102);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}

struct Range {
    start: i64,
    length: i64,
    dif: i64,
}

pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day5_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;

    let seeds = contents.lines().nth(0)
                        .unwrap()
                        .split(':')
                        .nth(1)
                        .unwrap()
                        .split_whitespace()
                        .map(|x| x.parse::<i64>()
                        .unwrap());
    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();    

    let mut skip = 3;
    let num_lines = contents.lines().count();
    while num_lines > skip{
        let tmp1 = get_next_block(contents.clone(), skip);
        maps.push(tmp1.clone());
        skip += tmp1.len() + 2;
    }
    let mut lowest: i64 = i64::MAX;
    let mut seed_ranges: Vec<Range> = Vec::new();
    let mut prev = -1;
    seeds.for_each(|seed|{
        //check if the current seed is the lowest dist in part1
        let test = get_dist(seed,maps.clone());
        if test < lowest{
            lowest = test;
        }
        //Build the set of ranges for part2
        if prev == -1{
            prev = seed;
        } else {
            let tmp = Range {
                start: prev,
                length: seed,
                dif: 0,
            };
            seed_ranges.push(tmp);
            prev = -1;
        }
    });
    
    seed_ranges.sort_by(|a,b|{
        a.start.cmp(&b.start)
    });

    let map_ranges: Vec<Vec<Range>> = maps.into_iter().map(|map|{
        let mut tmp_range:Vec<Range> = map.iter().map(|range|{
            Range {
                start: *range.get(1).unwrap_or(&0),
                length: *range.get(2).unwrap_or(&0),
                dif: range.get(0).unwrap_or(&0) - range.get(1).unwrap_or(&0),
            }
        }).collect::<Vec<Range>>();
        tmp_range.sort_by(|a,b|{
            a.start.cmp(&b.start)
        });
        tmp_range
    }).collect();
    
    let mut tmp:Vec<Range> = seed_ranges;
    map_ranges.iter().for_each(|map|{
        tmp = convert_range(&tmp, map);
    });
    let mut min_val: i64 = i64::MAX;
    tmp.iter().for_each(|val|{
        if val.start < min_val{
            min_val = val.start;
        }
    });
    Ok((lowest,min_val))
}
fn convert_range(input: &Vec<Range>, map: &Vec<Range>) -> Vec<Range>{
    let mut tmp: Vec<Range> = Vec::new();
    for seed in input{
        let mut seed_start = seed.start;
        let mut seed_end = seed.start + seed.length-1;
        let mut seed_length = seed.length;
        let mut range_found = false;
        for range in map{
            let range_start = range.start;
            let range_end = range.start + range.length-1;
            if seed_start >= range_start && seed_start <= range_end{
                if seed_end <= range_end && seed_end >= range_start{
                    let tmp_range = Range{
                        start: seed_start + range.dif,
                        length: seed_length,
                        dif: 0,
                    };
                    tmp.push(tmp_range);
                    range_found = true;
                } 
                else {
                    let tmp_range = Range{
                        start: seed_start + range.dif,
                        length: range_end - seed_start +1,
                        dif: 0,
                    };
                    seed_start = seed_start + tmp_range.length;
                    seed_length = seed_length - tmp_range.length; 
                    seed_end = seed_start + seed_length - 1;
                tmp.push(tmp_range);     
                }
            }
        }
        if !range_found{
            tmp.push(Range{
                start: seed_start,
                length: seed_length,
                dif: 0,
            });
        }
    }
    tmp
}


fn get_dist(seed: i64, maps: Vec<Vec<Vec<i64>>>) -> i64{
    let mut tmp = seed;
    for map in &maps{
        for val in map{
            let source = val.get(1).unwrap_or(&0);
            let dest = val.get(0).unwrap_or(&0);
            let range = val.get(2).unwrap_or(&0);
            let dif = dest - source;

            let bottom = source;
            let top = source + range -1;

            if tmp >= *bottom && tmp <= top{
                tmp += dif;
                break;
            }
         }
    }
    tmp
}

fn get_next_block(lines: String, skip: usize) -> Vec<Vec<i64>>{
    let result: Vec<Vec<i64>> = lines.lines()
                            .skip(skip)
                            .take_while(|&line| !line.trim().is_empty())
                            .map(|line| line.split_whitespace()
                            .filter_map(|s| s.parse::<i64>().ok())
                            .collect())
                            .collect();
    result    
}
