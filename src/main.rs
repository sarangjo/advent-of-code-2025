use crate::day1::day1;
use std::env;

pub mod day1;

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
    }
}
