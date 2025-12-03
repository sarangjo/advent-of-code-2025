use std::fs;

fn compute_possible(base: i64, len: u32) -> i64 {
    let possible = base * 10_i64.pow((len as u32) / 2) + base;
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
        let mut cur_len: u32;
        if len % 2 != 0 {
            // For odd numbers, the first possible number is going to be 10^(appropriate) repeated
            cur_len = len as u32 + 1;
            base = 10_i64.pow(cur_len / 2 - 1);
        } else {
            // First possible one is the first half repeated
            cur_len = len as u32;
            base = start[..len / 2].parse().unwrap();
        };
        // println!("base {}, cur_len {}", base, cur_len);

        // Move to our first possible number in range
        let mut possible = compute_possible(base, cur_len);
        while possible < start_num {
            // println!("possible < start :( {}", possible);
            base += 1;
            if base == 10 {
                cur_len += 2;
                // println!("base rollover. base {} cur_len {}", base, cur_len);
            }
            possible = compute_possible(base, cur_len);
        }
        while possible <= end_num {
            println!(
                "possible {} < end!!!!!!!!!!!!!!!!!!!!!!!!!!!! <<<<<<<<<",
                possible
            );
            total += possible;

            base += 1;
            if base == 10 {
                cur_len += 2;
                // println!("base rollover. base {} cur_len {}", base, cur_len);
            }
            possible = compute_possible(base, cur_len);
        }

        // println!("done");
    }

    println!("{}", total);
}
