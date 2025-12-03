use std::fs;

fn compute_possible(base: i64, half_len: u32) -> i64 {
    let possible = base * 10_i64.pow(half_len) + base;
    // println!("possible {}", possible);
    return possible;
}

pub fn day2(filename: &str) {
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
