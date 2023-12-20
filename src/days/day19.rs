use std::{io, collections::HashMap};
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day19_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,19114);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day19_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,167409079868000);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day19_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}
#[derive(Clone)]
struct Part {
    x:i64,
    m:i64,
    a:i64,
    s:i64,
}

#[derive(Clone)]
struct Instruction {
    unit:char,
    value:i64,
    destination:String,
    greater:bool,
}

#[derive(Clone)]
struct Range {
    x_min:i64,
    x_max:i64,
    m_min:i64,
    m_max:i64,
    a_min:i64,
    a_max:i64,
    s_min:i64,
    s_max:i64,
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;

    let (workflow_str, parts_str) = contents.split_at(contents.find("\n\n").unwrap());
    let parts: Vec<Part> = parts_str.trim_start_matches("\n").lines().map(|line|{
        let mut result = Part {
            x:0,
            m:0,
            a:0,
            s:0,
        };
        line.replace("{","").replace("}","").split(',').for_each(|val|{
            let (type_p, value) = val.split_at(val.find("=").unwrap());
            let value = value.replace("=","");
            match type_p {
                "x" => result.x = value.parse::<i64>().unwrap(), 
                "m" => result.m = value.parse::<i64>().unwrap(), 
                "a" => result.a = value.parse::<i64>().unwrap(), 
                "s" => result.s = value.parse::<i64>().unwrap(), 
                 _  => {}, 
            }
        });
        result 
    }).collect();
    let instructions = generate_hasmap(workflow_str);

    let mut part1 = 0;
    for part in parts{
        let mut cur_dest = "in".to_string();
        while cur_dest != "A" && cur_dest != "R" {
            let cur_inst = instructions.get(&cur_dest).unwrap();
            cur_dest = get_next_dest(part.clone(), cur_inst.to_vec());
        }
        if cur_dest == "A" {
            part1 += part.x + part.m + part.a + part.s;
        }
    }
    let mut range = Range{
        x_min:1,
        x_max:4000,
        m_min:1,
        m_max:4000,
        a_min:1,
        a_max:4000,
        s_min:1,
        s_max:4000,
    };
    let part2 = calc_ranges(instructions.clone(), &mut range, &"in");
    Ok((part1,part2))
}

fn calc_ranges(input: HashMap<String,Vec<Instruction>>, mut range: &mut Range, key: &str) -> i64{
    let mut result = 0;
    if key == "A" || key == "R" {
        if key == "A" {
            result = (range.x_max - range.x_min + 1)*(range.m_max - range.m_min + 1)*(range.a_max - range.a_min + 1)*(range.s_max - range.s_min + 1);
        }
        return result; 
    }
    let cur_inst = &input.get(key).unwrap();
    cur_inst.iter().for_each(|inst|{
        let mut tmp_range = range.clone();
        if inst.value == -1 {
            result += calc_ranges(input.clone(), &mut range, &inst.destination);
        } else {
            match inst.unit {
                'x' => {
                    if inst.greater {
                        tmp_range.x_min = inst.value + 1;
                        range.x_max = inst.value;
                    } else {
                        tmp_range.x_max = inst.value - 1;
                        range.x_min = inst.value;
                    }
                },
                'm' => {
                    if inst.greater {
                        tmp_range.m_min = inst.value + 1;
                        range.m_max = inst.value;
                    } else {
                        tmp_range.m_max = inst.value - 1;
                        range.m_min = inst.value;
                    }
                },
                'a' => {
                    if inst.greater {
                        tmp_range.a_min = inst.value + 1;
                        range.a_max = inst.value;
                    } else {
                        tmp_range.a_max = inst.value - 1;
                        range.a_min = inst.value;
                    }
                },
                's' => {
                    if inst.greater {
                        tmp_range.s_min = inst.value + 1;
                        range.s_max = inst.value;
                    } else {
                        tmp_range.s_max = inst.value - 1;
                        range.s_min = inst.value;
                    }
                },
                 _  => {}
            };
            result += calc_ranges(input.clone(), &mut tmp_range, &inst.destination);
        }
    });
    result
}

fn get_next_dest(part:Part, cur_inst: Vec<Instruction>) -> String{
    let mut cur_dest = "in".to_string();
    for inst in cur_inst{
        if inst.value == -1 {
            cur_dest = inst.destination.clone();
        } else {
            match inst.unit {
                'x' => {
                    if inst.greater{
                        if part.x > inst.value {
                            cur_dest = inst.destination.clone();
                            break;
                        }
                    } else {
                        if part.x < inst.value {
                            cur_dest = inst.destination.clone();
                            break;
                        }
                    }
                },
                'm' => {
                    if inst.greater{
                        if part.m > inst.value {
                            cur_dest = inst.destination.clone();
                            break;
                        }
                    } else {
                        if part.m < inst.value {
                            cur_dest = inst.destination.clone();
                            break;
                        }
                    }
                },
                'a' => {
                    if inst.greater{
                        if part.a > inst.value {
                            cur_dest = inst.destination.clone();
                            break;
                        }
                    } else {
                        if part.a < inst.value {
                            cur_dest = inst.destination.clone();
                            break;
                        }
                    }
                },
                's' => {
                    if inst.greater{
                        if part.s > inst.value {
                            cur_dest = inst.destination.clone();
                            break;
                        }
                    } else {
                        if part.s < inst.value {
                            cur_dest = inst.destination.clone();
                            break;
                        }
                    }
                },
                 _ => {},
            }
        }
    }
    cur_dest.clone()
}

fn generate_hasmap(input: &str) -> HashMap<String,Vec<Instruction>> {
    let mut instructions: HashMap<String, Vec<Instruction>> = HashMap::new();

    input.lines().for_each(|line|{
        let (key, vals) = line.split_at(line.find("{").unwrap());
        let inst:Vec<Instruction> = vals.replace("{","").replace("}","").split(',').map(|val|{
            let mut tmp = Instruction {
                destination:val.to_string(),
                unit:'X',
                value:-1,
                greater:false,
            };
            if val.contains("<") || val.contains(">"){
                if val.contains("<"){
                    tmp.greater = false;
                } else {
                    tmp.greater = true;
                }
                tmp.unit = val.chars().nth(0).unwrap();
                let (num_str, dest) = val.split_at(2).1.split_at(val.find(":").unwrap()-2);

                tmp.value = num_str.parse::<i64>().unwrap();
                tmp.destination = dest.replace(":","").to_string();
            } 
            tmp
        }).collect();
        instructions.insert(key.to_string(), inst);
    });
    instructions
}

#[allow(dead_code)]
fn print_hasmap(input: &HashMap<String,Vec<Instruction>>){
    for (key,value) in input{
        print_hash_line(key, value);
    }
}

#[allow(dead_code)]
fn print_hash_line(key: &str, value:&Vec<Instruction>){
    print!("{}{{",key);
    for inst in value{
        if inst.value == -1{
            print!("{}",inst.destination);
        } else {
            print!("{}",inst.unit);
            if inst.greater{
                print!(">");
            } else {
                print!("<");
            }
            print!("{}:{},",inst.value,inst.destination);
        }
    }
    println!("}}");
}
