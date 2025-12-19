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
