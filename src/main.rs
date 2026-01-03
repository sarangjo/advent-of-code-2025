use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use crate::day5::day5;
use crate::day6::day6;
use crate::day7::day7;
use crate::day8::day8;
use crate::day9::day9;
use std::env;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Specify day");
    }

    let day: i32 = args[1].parse().unwrap();
    let is_sample = args.len() == 3 && args[2] == "--sample";
    let filename = if is_sample {
        format!("sample{}.txt", day)
    } else {
        format!("day{}.txt", day)
    };

    if day == 1 {
        day1(&filename);
    } else if day == 2 {
        day2(&filename);
    } else if day == 3 {
        day3(&filename);
    } else if day == 4 {
        day4(&filename);
    } else if day == 5 {
        day5(&filename);
    } else if day == 6 {
        day6(&filename);
    } else if day == 7 {
        day7(&filename);
    } else if day == 8 {
        day8(&filename, is_sample);
    } else if day == 9 {
        day9(&filename);
    }
}
