use std::fs::File;
use std::io::{BufRead, BufReader};

fn runner<F>(filename: &str, f: F)
where
    F: Fn(i32, i32, i32) -> i32,
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

        count += f(val, direction, amount);
        val = (((val + direction * amount) % 100) + 100) % 100;
    }

    println!("{}", count);
}

pub fn day1(filename: &str) {
    println!("Filename: {}", filename);

    let part1 = |val: i32, direction: i32, amount: i32| -> i32 {
        return if (val + direction * amount) % 100 == 0 {
            1
        } else {
            0
        };
    };

    let part2 = |val: i32, direction: i32, amount: i32| -> i32 {
        let new_val = val + direction * amount;
        /*
        new_val | val = 0   | val != 0  | new_val / 100
        -250    | 2         | 3         | -2
        -200    | 2         | 3         | -2
        -150    | 1         | 2         | -1
        -100    | 1         | 2         | -1
        -50     | 0         | 1         | 0

        0       | 0         | 1         | 0

        50      | 0         | 0         | 0
        100     | 1         | 1         | 1
        150     | 1         | 1         | 1
        200     | 2         | 2         | 2
        250     | 2         | 2         | 2
        */
        return if new_val > 0 {
            new_val / 100
        } else if new_val == 0 {
            if val == 0 { 0 } else { 1 }
        } else {
            -(new_val / 100) + (if val == 0 { 0 } else { 1 })
        };
    };

    runner(filename, part1);
    runner(filename, part2);
}
