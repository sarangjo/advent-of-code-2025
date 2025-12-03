use std::fs::File;
use std::io::{BufRead, BufReader};

fn runner<F>(filename: &str, f: F)
where
    F: Fn(i32, i32, i32) -> (i32, i32),
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut val = 50;

    for line_result in reader.lines() {
        let line = line_result.unwrap();

        let direction = if line.chars().nth(0).unwrap() == 'L' {
            -1
        } else {
            1
        };
        let amount: i32 = line.trim()[1..].parse().unwrap();

        let (new_val, count_change) = f(val, direction, amount);
        val = new_val;
        count += count_change;
    }

    println!("{}", count);
}

pub fn day1(filename: &str) {
    println!("Filename: {}", filename);

    let part1 = |val: i32, direction: i32, amount: i32| -> (i32, i32) {
        let val2 = (val + direction * amount) % 100;

        return (val2, if val == 0 { 1 } else { 0 });
    };

    let part2 = |val: i32, direction: i32, amount: i32| -> (i32, i32) {
        let mut new_val = val + direction * amount;
        // TODO this behavior is different in rust: -18/100 = 0, not -1 as in Python. So figure out
        // how to handle that gracefully and adjust our adjustments below
        let mut cross_times = ((new_val / 100) - (val / 100)).abs();
        new_val %= 100;

        if new_val == 0 && direction == -1 {
            // If we land at 0 and we were direction == -1, then we need to add 1 to cross_times
            cross_times += 1;
        } else if val == 0 && direction == -1 {
            cross_times -= 1;
        }

        return (new_val, cross_times);
    };

    runner(filename, part1);
    runner(filename, part2);
}
