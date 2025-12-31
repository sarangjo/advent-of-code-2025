use std::{
    cmp::min,
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

fn compute_column(lines: &Vec<String>, start_idx: usize, end_idx: usize, oper: Operator) -> u64 {
    let nums = lines.iter().map(|line| {
        line[start_idx..(min(line.len(), end_idx))]
            .trim()
            .parse::<u64>()
            .unwrap()
    });
    match oper {
        Operator::Sum => nums.sum(),
        Operator::Product => nums.product(),
    }
}

fn part1(filename: &str) {
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
            total += compute_column(&lines, start_idx, i - 1, oper);

            oper = Operator::from(c);
            start_idx = i;
        }
    }

    // Fencepost
    total += compute_column(&lines, start_idx, max_len, oper);

    println!("total: {}", total);
}

pub fn day6(filename: &str) {
    part1(filename);
}
