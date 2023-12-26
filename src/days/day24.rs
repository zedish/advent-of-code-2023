use std::io;
use crate::utils;
use z3::ast::{Ast, Int, Real};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day24_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,2);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_example1() {
        let result = do_puzzle("day24_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,47);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part1_solve() {
        let result = do_puzzle("day24_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,11995);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_solve() {
        let result = do_puzzle("day24_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,983620716335751);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}

#[derive(Clone)]
struct Line {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day24_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let lines:Vec<Line> = contents.lines().map(|line|{
        let line_tmp = line.replace(" ", "");
        let points:Vec<&str> = line_tmp.split('@').nth(0).unwrap().split(',').collect();
        let vel:Vec<&str> = line_tmp.split('@').nth(1).unwrap().split(',').collect();
        Line {
            px: points.get(0).unwrap().parse::<f64>().unwrap(),
            py: points.get(1).unwrap().parse::<f64>().unwrap(),
            pz: points.get(2).unwrap().parse::<f64>().unwrap(),
            vx: vel.get(0).unwrap().parse::<f64>().unwrap(),
            vy: vel.get(1).unwrap().parse::<f64>().unwrap(),
            vz: vel.get(2).unwrap().parse::<f64>().unwrap(),
        }
    }).collect();

    let mut lower_val = 200000000000000.0;
    let mut upper_val = 400000000000000.0;
    if input.contains("_1_0.txt") {
        lower_val = 7.0;
        upper_val = 27.0;
    }
    let mut intersections = 0;
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            let line1 = lines.get(i).unwrap();
            let line2 = lines.get(j).unwrap();
            if let Some((x, y)) = check_intercept(convert_line_tomxb(&line1), convert_line_tomxb(&line2)) {
                if x >= lower_val && y >= lower_val &&
                   x <= upper_val && y <= upper_val {
                    if check_past(&line1, (x,y)) && check_past(&line2, (x,y)){
                        intersections += 1;
                    }
                }
            } 
        }
    }
    let part2 = part2_solve(lines);

    Ok((intersections as i64,part2 as i64))
}
fn part2_solve(lines: Vec<Line>) -> usize {
    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);
    let [fx,fy,fz,fdx,fdy,fdz] = ["fx","fy","fz","fdx","fdy","fdz"].map(|v| Real::new_const(&ctx, v));

    let zero = Int::from_i64(&ctx, 0).to_real();
    for (i, line) in lines.iter().enumerate() {
        let [x,y,z,dx,dy,dz] = [line.px,line.py,line.pz,line.vx,line.vy,line.vz].map(|v| Int::from_i64(&ctx, v as _).to_real());
        let t = Real::new_const(&ctx, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(s.check(), z3::SatResult::Sat);
    let res = s.get_model().unwrap().eval(&(&fx + &fy + &fz), true).unwrap();
    res.to_string().strip_suffix(".0").unwrap().parse().unwrap()
}

fn check_past(input_line: &Line, (x,y):(f32,f32)) -> bool{
    let x_delta = x - input_line.px as f32;
    let y_delta = y - input_line.py as f32;

    let x_valid = x_delta < 0.0 && input_line.vx < 0.0 || x_delta > 0.0 && input_line.vx > 0.0;
    let y_valid = y_delta < 0.0 && input_line.vy < 0.0 || y_delta > 0.0 && input_line.vy > 0.0;

    x_valid && y_valid
}

fn check_intercept(line1:(f32,f32),line2:(f32,f32)) -> Option<(f32,f32)>{
    if line1.1 == line2.1 { //if lines are parallel
        return None;
    }
    let x = (line2.0 - line1.0) / (line1.1 - line2.1);
    let y = line1.1 * x + line1.0; // Substitute x into either line equation

    Some((x, y))
}

fn convert_line_tomxb(input: &Line) -> (f32,f32) {
    let slope = input.vy as f32/input.vx as f32;
    let c = input.py as f32 - (slope * input.px as f32);
    // println!("y={}x+{}",slope,c);
    (c as f32, slope)
}

#[allow(dead_code)]
fn print_line(input: Line){
    println!("{},{},{} -> {},{},{}",input.px,input.py,input.pz,input.vx,input.vy,input.vz);
}
