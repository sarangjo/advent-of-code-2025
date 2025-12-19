use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn day3(filename: &str) {
    part1(filename);
    part2(filename);
}

fn part1(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;

    for line_result in reader.lines() {
        let line = line_result.unwrap();

        // Two-pass approach. First patch to find the best first digit, the second to find the best second
        let mut best_first: (usize, i32) = (0, -1);
        for (i, c) in line
            .chars()
            .enumerate()
            .map(|tup| (tup.0, tup.1.to_digit(10).unwrap() as i32))
        {
            if i == line.len() - 1 {
                break;
            }

            if best_first.1 < c {
                best_first = (i, c);
            }
        }
        println!("best first {:?}", best_first);

        let mut best_second: (usize, i32) = (0, -1);
        for (i, c) in line
            .chars()
            .skip(best_first.0 + 1)
            .enumerate()
            .map(|tup| (tup.0, tup.1.to_digit(10).unwrap() as i32))
        {
            if best_second.1 < c {
                best_second = (i + best_first.0 + 1, c);
            }
        }

        println!("best second {:?}", best_second);

        total += best_first.1 * 10 + best_second.1;
    }

    println!("total: {}", total);
}

fn part2(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: i64 = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();

        let line_as_nums: Vec<i64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();

        let mut nums = [0_i64; 12];

        let mut last_idx: i32 = -1;
        for i in 0..12 {
            let mut cur_best: i64 = -1;
            for j in ((last_idx + 1) as usize)..line_as_nums.len() - 12 + i + 1 {
                if line_as_nums[j] > cur_best {
                    last_idx = j as i32;
                    cur_best = line_as_nums[j];
                }
            }
            nums[i] = cur_best;
        }

        let mut num_str = String::new();
        for digit in nums {
            num_str += &digit.to_string();
        }
        let num: i64 = num_str.parse().unwrap();

        total += num;
    }

    println!("total: {}", total);
}
