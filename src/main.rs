use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use std::env;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Specify day");
    }

    let day: i32 = args[1].parse().unwrap();
    let filename = if args.len() == 3 && args[2] == "--sample" {
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
    }
}
