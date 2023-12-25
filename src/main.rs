use std::time::Instant;
mod utils;
mod days;

fn main() {
    println!("Hello, Advent of Code!");
    
    let skip: Vec<i32> = vec![0];

    let days = 23..=23;
    for day in days{
        if skip.contains(&day){continue;}
        let func = get_day_solver(day);
        println!("============== Day:{} ==============",day);
        let time = Instant::now();
        let result = func();
        let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
        println!("Part1:{}        Part2:{}\tTime for solve:{}ms",result.0,result.1,elapsed_ms);

    }
}

fn get_day_solver(day: i32) -> fn() -> (i64,i64) {
    match day {
         1 => days::day01::solve,
         2 => days::day02::solve,
         3 => days::day03::solve,
         4 => days::day04::solve,
         5 => days::day05::solve,
         6 => days::day06::solve,
         7 => days::day07::solve,
         8 => days::day08::solve,
         9 => days::day09::solve,
        10 => days::day10::solve,
        11 => days::day11::solve,
        12 => days::day12::solve,
        13 => days::day13::solve,
        14 => days::day14::solve,
        15 => days::day15::solve,
        16 => days::day16::solve,
        17 => days::day17::solve,
        18 => days::day18::solve,
        19 => days::day19::solve,
        20 => days::day20::solve,
        21 => days::day21::solve,
        22 => days::day22::solve,
        23 => days::day23::solve,
        24 => days::day24::solve,
        25 => days::day25::solve,
         _ => unimplemented!(),
    }
}
