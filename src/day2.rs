use std::fs;

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
    let content = fs::read_to_string(filename).unwrap();

    let mut total = 0;

    for range_str in content.split(',') {
        let mut range = range_str.split('-');

        println!("current range: {}", range_str);

        let start = range.next().unwrap().trim();
        let end = range.next().unwrap().trim();

        let start_num: i32 = start.parse().unwrap();
        let end_num: i32 = start.parse().unwrap();

        // Go by length. We start with start's length and evaluate which possible repeats it could have
        // and continue until we cross end. Then repeat for each possible repeat breakdown.

        // 999
        let full_len = start.len(); // 3

        // Highest possible piece length is full_len/2 (splitting into two pieces, piece_count=2)
        for piece_len in 1..(full_len / 2 + 1) {
            // Only look for real divisions
            if full_len % piece_len != 0 {
                continue;
            }
            let piece_count = full_len / piece_len;

            // Pick the prefex of length `piece_len` and find the best one according to start
            // e.g. piece_len = 1
            // base = 9
            let mut base: i32 = start[..piece_len].parse().unwrap();

            // Construct the number by repeating `base` `piece_count` times
            let construct_num = |base: i32| {
                let mut _n = String::new();
                for _ in 1..piece_count {
                    _n += base.to_string().as_str();
                }
                return _n.parse::<i32>().unwrap();
            };

            let mut num = construct_num(base);
            // Now we have our possible number that is a repeat of the piece. It's possible
            // that this is smaller than our actual number, e.g. the full number is 488 but num is
            // 444. So we keep reconstructing our number by bumping base until we get to a value that's
            // larger than start
            while num < start_num {
                base += 1;
            }
        }
    }

    println!("{}", total);
}
