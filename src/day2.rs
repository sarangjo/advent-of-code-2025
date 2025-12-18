use std::{collections::HashSet, fs};

pub fn day2(filename: &str) {
    part1(filename);
    part2(filename);
}

fn compute_possible(base: i64, half_len: u32) -> i64 {
    let possible = base * 10_i64.pow(half_len) + base;
    // println!("possible {}", possible);
    return possible;
}

fn part1(filename: &str) {
    let content = fs::read_to_string(filename).unwrap();

    let mut total = 0;

    for range_str in content.split(',') {
        let mut range = range_str.split('-');

        println!("range_str: {}", range_str);

        let start = range.next().unwrap().trim();
        let start_num: i64 = start.parse().unwrap();
        let end_num: i64 = range.next().unwrap().trim().parse().unwrap();

        // Based on length of start, we evaluate the first possible duplicate
        let len: usize = start.len();

        let mut base: i64;
        // half_len is the length of the half number we're considering that gets duplicated
        let mut half_len: u32;
        if len % 2 != 0 {
            // For odd numbers, the first possible number is going to be 10^(appropriate) repeated
            half_len = (len + 1) as u32 / 2;
            base = 10_i64.pow(half_len - 1);
        } else {
            // First possible one is the first half repeated
            half_len = len as u32 / 2;
            base = start[..len / 2].parse().unwrap();
        };
        // println!("base {}, cur_len {}", base, cur_len);
        let mut possible = compute_possible(base, half_len);

        // Helper closure to bump base and len, and returns the new possible
        let mut base_len_bumper = || {
            // println!("possible < start :( {}", possible);
            base += 1;
            if base.to_string().len() != half_len as usize {
                half_len += 1;
                // println!("base rollover. base {} cur_len {}", base, cur_len);
            }
            return compute_possible(base, half_len);
        };

        // Move to our first possible number in range
        while possible < start_num {
            possible = base_len_bumper();
        }
        while possible <= end_num {
            println!(
                "possible {} < end!!!!!!!!!!!!!!!!!!!!!!!!!!!! <<<<<<<<<",
                possible
            );
            total += possible;

            possible = base_len_bumper();
        }

        // println!("done");
    }

    println!("{}", total);
}

fn part2(filename: &str) {
    let construct_num = |base: i64, piece_count: u32| {
        let num_str = (1..piece_count + 1)
            .map(|_| base.to_string())
            .collect::<String>();
        let res = num_str.parse::<i64>();
        match res {
            Ok(ans) => ans,
            Err(err) => {
                println!("Encountered parse error for {}... why?", err);
                -1
            }
        }
    };

    let content = fs::read_to_string(filename).unwrap();

    let mut total = 0;

    for range_str in content.split(',') {
        let mut range = range_str.split('-');
        let mut solutions = HashSet::new();

        let start = range.next().unwrap().trim();
        let end = range.next().unwrap().trim();

        println!(">> current range: {}-{} <<", start, end);

        // as numbers
        let start_num: i64 = start.parse().unwrap();
        let end_num: i64 = end.parse().unwrap();

        // Highest possible piece length is full_len/2 (splitting into two pieces, piece_count=2)
        // +1 because the range we want is [1, end.len()/2] which is equivalent to [1, end.len()/2+1)
        for piece_len in 1..(end.len() / 2 + 1) {
            let min_piece_count =
                (((start.len() as f32) / (piece_len as f32)) as f32).ceil() as u32;
            let max_piece_count = (((end.len() as f32) / (piece_len as f32)) as f32).ceil() as u32;

            let base_max = 10_i64.pow(piece_len as u32);

            for piece_count in min_piece_count..max_piece_count + 1 {
                // Has to repeat at least twice
                if piece_count == 1 {
                    continue;
                }

                // Pick the prefex of length `piece_len` and find the best one according to start
                // e.g. piece_len = 1
                // base = 4
                let mut base: i64 = 10_i64.pow(piece_len as u32 - 1);

                /*  if start.len() % piece_len == 0 {
                    // start[..piece_len].parse().unwrap()
                } else {
                };*/

                let mut num = construct_num(base, piece_count);
                // Now we have our possible number that is a repeat of the piece. It's possible
                // that this is smaller than our actual number, e.g. the full number is 488 but num is
                // 444. So we keep reconstructing our number by bumping base until we get to a value that's
                // larger than start
                while num < start_num {
                    base += 1;
                    if base == base_max {
                        break;
                    }
                    num = construct_num(base, piece_count);
                }

                while num <= end_num {
                    if solutions.insert(num) {
                        println!("Found: {} (piece_len {})", num, piece_len);
                    }

                    base += 1;
                    if base == base_max {
                        break;
                    }
                    num = construct_num(base, piece_count);
                }
            }
        }

        // Add to total
        for sol in &solutions {
            total += sol;
        }
    }

    println!("{}", total);
}
