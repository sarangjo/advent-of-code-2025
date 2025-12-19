use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn day3(filename: &str) {
    part1(filename);
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

fn part2() {
    let total = 0;

    // for line_result in reader.lines() {
        // let line = line_result.unwrap();
        let line = String::from("234234234234278");
        
        let line_as_nums: Vec<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        
        println!("line_as_nums {:?}", line_as_nums);
        
        let mut nums = [0; 12];
        
        let mut last_idx: i32 = -1;
        for i in 0..12 {
            println!("i {}", i);
            let mut cur_best = -1;
            for j in ((last_idx+1) as usize)..line_as_nums.len() - 12 + i + 1 {
                println!("j {}", j);
                if line_as_nums[j] > cur_best {
                    println!("setting last_idx to {} and cur_best to {}", j, line_as_nums[j]);
                    last_idx = j as i32;
                    cur_best = line_as_nums[j];
                }
            }
            nums[i] = cur_best;
        }
        
        println!("nums {:?}", nums);
    // }

    println!("total: {}", total);
}
