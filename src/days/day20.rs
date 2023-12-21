use std::{io, collections::HashSet};
use crate::utils;
use std::collections::HashMap;
use gcd::Gcd;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day20_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,32000000);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_example2() {
        let result = do_puzzle("day20_1_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,11687500);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_solve() {
        let result = do_puzzle("day20_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,681194780);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_solve() {
        let result = do_puzzle("day20_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,238593356738827);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}

pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day20_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

#[derive(Clone, PartialEq)]
enum ModType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Clone)]
struct Module {
    mod_type: ModType,
    destinations: Vec<String>,
    mod_name: String,
    state: bool,
    inputs: HashMap<String,bool>,
}

#[derive(Clone, PartialEq)]
struct Pulse {
    src: String,
    dest: String,
    state: bool,
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;

    let mut graph: HashMap<String,Module> = contents.lines().map(|line|{
        let tmp = gen_module(line);
        (tmp.mod_name.clone(),tmp)
    }).collect();
    
    update_conj_updates(&mut graph);
    
    let mut important = get_important_nodes(&graph);
   
    let mut pulse_nums = (0,0);
    let mut press_num = 0;
    for i in 0..10000{
        let (tmp_high,tmp_low) = push_button(&mut graph, &mut press_num, &mut important);
        if i < 1000{
            pulse_nums.0 += tmp_high;
            pulse_nums.1 += tmp_low;
        }
    }
    let part1 = pulse_nums.0*pulse_nums.1;
    
    let loops = important.iter().map(|(_,val)|{*val as u64}).collect::<Vec<u64>>();
    let part2 = lcm_of_multiple(&loops);

    Ok((part1,part2 as i64))
}

fn get_mod_type(input: char) -> ModType {
    match input {
        '%' => ModType::FlipFlop,
        '&' => ModType::Conjunction,
         _  => ModType::Broadcaster,
    }
}

fn gen_module(input: &str) -> Module {
    Module {
        mod_type: get_mod_type(input.chars().next().unwrap()),
        mod_name: input.split_whitespace().next().unwrap().to_string().replace("%", "").replace("&", ""),
        destinations: (input.replace(" ","").split_at(input.find(">").unwrap()).1).split(',').map(|val|{val.to_string()}).collect(),
        state: false,
        inputs: HashMap::new(),
    } 
}

#[allow(dead_code)]
fn print_module(input: Module) {
    match input.mod_type {
        ModType::Broadcaster => print!("*"),
        ModType::FlipFlop => print!("%"),
        ModType::Conjunction => print!("&"),
    }
    print!("{} -> ",input.mod_name);
    input.destinations.iter().for_each(|val|{
        print!(",{}",val);
    });
    print!("   State:{}\tInputs: ",input.state);
    input.inputs.iter().for_each(|(key,val)|{
        print!("{}:{}  ",key,val);
    });
    println!("");
}

fn update_conj_updates(input: &mut HashMap<String,Module>){
    let mut conjs: Vec<String> = Vec::new();
    for(key, val) in input.clone() {
        if val.mod_type == ModType::Conjunction{
            conjs.push(key.to_string());
        } 
    }
    for (key, val) in input.clone() {
        for dest in &val.destinations {
            if conjs.contains(&dest) {
                input.get_mut(dest).unwrap().inputs.insert(key.to_string(), false);
            }
        }
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / a.gcd(b)) * b
}

fn lcm_of_multiple(numbers: &[u64]) -> u64 {
    if numbers.len() < 2 {
        return 0
    }

    let mut result = numbers[0];
    for &num in &numbers[1..] {
        result = lcm(result, num);
    }
    result
}

fn get_important_nodes(graph: &HashMap<String,Module>) -> HashMap<String,u64> {
    let mut result: HashMap<String,u64> = HashMap::new();
    graph.iter().for_each(|(_,val)|{
        if val.destinations.contains(&"rx".to_string()){
            // println!("important:{:?}",val.inputs);
            val.inputs.iter().for_each(|(key,_)|{
                result.insert(key.clone(), 0);
            });
        }
    });
    result
}

fn count_pulses(pulses: &Vec<Pulse>) -> (i64,i64) {
    let mut num_high = 0;
    let mut num_low = 0;
    pulses.iter().for_each(|pulse|{
        if pulse.state {
            num_high += 1;
        } else {
            num_low += 1;
        }
    });
    (num_high,num_low)
}

fn push_button(graph: &mut HashMap<String,Module>, press_num: &mut u64, important: &mut HashMap<String,u64>) -> (i64,i64) {
    *press_num += 1;
    let mut pulses: Vec<Pulse> = Vec::new();
    graph.get("broadcaster").unwrap().destinations.iter().for_each(|dest|{
        pulses.push(Pulse {
            src: "broadcaster".to_string(),
            dest: dest.to_string(),
            state: false,
        }); 
    });
    let mut pulse_nums = (0,1);//start low at 1 for pushing the button
    
    while pulses.len() > 0 {
        for pulse in &pulses{
            if pulse.dest == "bq" && pulse.state == true{
                if *important.get_mut(&pulse.src).unwrap() == 0{
                    *important.get_mut(&pulse.src).unwrap() = *press_num;
                }
            }
        }
        let (tmp_high,tmp_low) = count_pulses(&pulses);
        pulse_nums.0 += tmp_high;
        pulse_nums.1 += tmp_low;
        pulses = calculate_pulse(pulses, graph);
    }
    pulse_nums
}

fn calculate_pulse(pulses: Vec<Pulse>, graph: &mut HashMap<String,Module>) -> Vec<Pulse> {
    let mut new_pulses:Vec<Pulse> = Vec::new();
    let mut conjunctions:HashSet<String> = HashSet::new();
    pulses.iter().for_each(|pulse|{
        if let Some(tmp_mod) = graph.get_mut(&pulse.dest){
            match tmp_mod.mod_type {
                    ModType::FlipFlop => {
                        if !pulse.state {
                            tmp_mod.state = !tmp_mod.state;
                            tmp_mod.destinations.iter().for_each(|dest|{
                                new_pulses.push(Pulse {
                                    dest: dest.to_string(),
                                    state: tmp_mod.state,
                                    src: pulse.dest.clone(),
                                });
                            });
                        }
                    },
                    ModType::Conjunction => {
                        *tmp_mod.inputs.get_mut(&pulse.src).unwrap() = pulse.state;
                        conjunctions.insert(pulse.dest.clone());
            
                        let mut state_out = true;
                        tmp_mod.inputs.iter().for_each(|input|{
                            state_out = state_out && *input.1; 
                        });
                        tmp_mod.destinations.iter().for_each(|dest|{
                            new_pulses.push(Pulse {
                                dest: dest.to_string(),
                                state: !state_out,
                                src: tmp_mod.mod_name.clone(),
                            });
                        });
                    },
                    _ => {},
            }
        } 
    });
    new_pulses 
}
