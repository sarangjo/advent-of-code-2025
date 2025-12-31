use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

enum Operator {
    Sum,
    Product,
}

impl Operator {
    fn from(c: char) -> Self {
        if c == '*' {
            Operator::Product
        } else {
            Operator::Sum
        }
    }
}

fn part1(lines: &Vec<String>, start_idx: usize, end_idx: usize, oper: Operator) -> u64 {
    let nums = lines.iter().map(|line| {
        line[start_idx..(cmp::min(line.len(), end_idx))]
            .trim()
            .parse::<u64>()
            .unwrap()
    });
    match oper {
        Operator::Sum => nums.sum(),
        Operator::Product => nums.product(),
    }
}

fn part2(lines: &Vec<String>, start_idx: usize, end_idx: usize, oper: Operator) -> u64 {
    // For part 2, we go right to left from end_idx-1 to start_idx
    let mut res = match oper {
        Operator::Sum => 0,
        Operator::Product => 1,
    };

    for i in (start_idx..end_idx).rev() {
        let mut num_str = String::new();
        for line in lines {
            if i >= line.len() || line.chars().nth(i).unwrap() == ' ' {
                continue;
            }
            num_str.push(line.chars().nth(i).unwrap());
        }
        let num: u64 = num_str.parse().unwrap();

        match oper {
            Operator::Sum => {
                res += num;
            }
            Operator::Product => {
                res *= num;
            }
        }
    }

    return res;
}

fn runner<F>(filename: &str, column_computer: F)
where
    F: Fn(&Vec<String>, usize, usize, Operator) -> u64,
{
    // Slurp up file contents into vec of strings so we can parse the last one
    let mut lines = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let oper_line = lines.pop().unwrap();

    // Precompute the longest line length
    let mut max_len = 0;
    for l in &lines {
        if l.len() > max_len {
            max_len = l.len();
        }
    }

    let mut total = 0_u64;

    let mut start_idx = 0;
    let mut oper = Operator::Sum;

    for (i, c) in oper_line.char_indices() {
        if i == 0 {
            oper = Operator::from(c);
            continue;
        }

        if c != ' ' {
            // Found a character, column is done
            total += column_computer(&lines, start_idx, i - 1, oper);

            oper = Operator::from(c);
            start_idx = i;
        }
    }

    // Fencepost
    total += column_computer(&lines, start_idx, max_len, oper);

    println!("total: {}", total);
}

pub fn day6(filename: &str) {
    runner(filename, part1);
    runner(filename, part2);
}
