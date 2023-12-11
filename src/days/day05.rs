use std::io;
use crate::utils;
use std::io::Write;

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
}


// struct Range {
//     start: i64,
//     length: i64,
//     dif: i64,
// }

pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day5_1_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

// fn do_puzzle2(input: &str)-> Result<(i64,i64), io::Error>{
//     let contents = utils::read_file(input)?;
//     let progress = "/-\\-"; 
//
//     let seeds = contents.lines().nth(0)
//                         .unwrap()
//                         .split(':')
//                         .nth(1)
//                         .unwrap()
//                         .split_whitespace()
//                         .map(|x| x.parse::<i64>()
//                         .unwrap());
//     let mut seed_vec: Vec<i64> = Vec::new();
//     for seed in seeds.clone(){
//         seed_vec.push(seed);
//     }
//     let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();    
//
//     let mut skip2 = 3;
//     let num_lines = contents.lines().count();
//     while num_lines > skip2{
//         let tmp1 = get_next_block(contents.clone(), skip2);
//         maps.push(tmp1.clone());
//         skip2 += tmp1.len() + 2;
//     }
//     // print!("Seed");
//     // for seed in &seed_vec{
//     //     print!(":{}",seed);
//     // }
//     // println!("");
//     
//     // for map in &maps{
//     //     println!("map:");
//     //     for range in map{
//     //         for val in range{
//     //             print!(":{}",val);
//     //         }
//     //         println!("");
//     //     }
//     //     println!("");
//     // }
//     //
//     let mut seed_ranges: Vec<Range> = Vec::new();
//     
//     let mut prev = -1;
//     for seed in seed_vec{
//         if prev == -1{
//             prev = seed;
//         } else {
//             let tmp = Range {
//                 start: prev,
//                 length: seed,
//                 dif: 0,
//             };
//             seed_ranges.push(tmp);
//             prev = -1;
//         }
//     }
//     seed_ranges.sort_by(|a,b|{
//         a.start.cmp(&b.start)
//     });
//     println!("Seed range:");
//     for seeds in &seed_ranges{
//         println!("Start:{} Len:{} Dist:{}",seeds.start,seeds.length,seeds.dif);
//     }
//
//     let mut map_ranges: Vec<Vec<Range>> = Vec::new();
//     for map in maps{
//         let mut tmp_range:Vec<Range> = Vec::new();
//         for range in map{
//             let tmp = Range {
//                 start: *range.get(1).unwrap_or(&0),
//                 length: *range.get(2).unwrap_or(&0),
//                 dif: range.get(0).unwrap_or(&0) - range.get(1).unwrap_or(&0),
//             };
//             tmp_range.push(tmp);    
//         }
//
//         tmp_range.sort_by(|a,b|{
//             a.start.cmp(&b.start)
//         });
//         map_ranges.push(tmp_range);
//     }
//     for map in &map_ranges{
//         println!("Map range:");
//         for range in map{
//             println!("Start:{} Len:{} Dist:{}",range.start,range.length,range.dif);
//         }
//     }
//
//     println!("-------");
//     for map in map_ranges{
//         let mut tmp:Vec<Range> = Vec::new();
//         for seed in &seed_ranges{
//             let seed_start = seed.start;
//             let seed_end = seed.start + seed.length-1;
//             for range in &map{
//                 let range_start = range.start;
//                 let range_end = range.start + range.length-1;
//                 //TODO: cover the case where the see is less then all ranges:q
//                 if seed_start >= range_start && seed_start <= range_end{
//                     if seed_end <= range_end && seed_end >= range_start{
//                         let tmp_range = Range{
//                             start: seed_start + range.dif,
//                             length: seed.length,
//                             dif: 0,
//                         };
//                         tmp.push(tmp_range);
//                     }
//                 }
//             } 
//         }
//         for val in tmp{
//             println!("Start:{} Len:{} Dist:{}",val.start,val.length,val.dif);
//         }
//     }
//     Ok((0,0))
// }

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let progress = "/-\\-"; 

    let seeds = contents.lines().nth(0)
                        .unwrap()
                        .split(':')
                        .nth(1)
                        .unwrap()
                        .split_whitespace()
                        .map(|x| x.parse::<i64>()
                        .unwrap());
    let mut seed_vec: Vec<i64> = Vec::new();
    for seed in seeds.clone(){
        seed_vec.push(seed);
    }
    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();    

    let mut skip2 = 3;
    let num_lines = contents.lines().count();
    while num_lines > skip2{
        let tmp1 = get_next_block(contents.clone(), skip2);
        maps.push(tmp1.clone());
        skip2 += tmp1.len() + 2;
    }
 
    let mut lowest: i64 = i64::MAX;
    for seed in seeds.clone(){
        let test = get_dist(seed,maps.clone());
        if test < lowest{
            lowest = test;
        }
    }
    let mut part2: i64 = i64::MAX;
    let max_seed = get_max_seed(seed_vec.clone());
    let mut thing = 0;
    for i in 0..=max_seed{
        let tmp_seed = get_seed(i, maps.clone());
        if check_valid_seed(tmp_seed, seed_vec.clone()){
            part2 = i;
            break;
        }
        thing += 1;
        if thing > 50000{
            thing = 0;
            print!("\r{}\tSeed:{}",progress.chars().nth((i%4)as usize).unwrap(),i);
            std::io::stdout().flush().unwrap();
        }
    }
    print!("\r");
    std::io::stdout().flush().unwrap();
    let val1:i32 = lowest.try_into().unwrap();
    let val2:i32 = part2.try_into().unwrap(); 
    Ok((val1 as i64,val2 as i64))
}
fn get_max_seed(seeds: Vec<i64>) -> i64{
    let mut prev = -1;
    let mut max = 0;
    for seed in seeds{
        if prev == -1{
            prev = seed;
        } else {
            let val = prev + seed;
            if val > max{
                max = val
            }
            prev = -1
        }
    }
    max
}

fn check_valid_seed(input: i64, seeds: Vec<i64>) -> bool{
    let mut prev = -1;
    let mut valid = false;
    for seed in seeds{
        if prev == -1{
            prev = seed;
        } else {
            if input >= prev && input < prev + seed{
                valid = true;
            }
            prev = -1
        }
    }

    valid
}

fn get_seed(seed: i64, maps: Vec<Vec<Vec<i64>>>) -> i64{
    let mut tmp = seed;
    for map in maps.iter().rev(){
        for val in map{
            let dest = val.get(1).unwrap_or(&0);
            let source = val.get(0).unwrap_or(&0);
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
                            .map(|line| line.split_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect())
                            .collect();
    result    
}
