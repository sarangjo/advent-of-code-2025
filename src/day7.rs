use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn part1(filename: &str) {
    let lines = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap());

    let mut tachyons = HashSet::new();

    let mut tachyon_count = 0;

    for line in lines {
        let mut new_tachyons = HashSet::new();
        for (i, c) in line.char_indices() {
            if c == 'S' {
                new_tachyons.insert(i);
                break;
            }

            if tachyons.contains(&i) {
                if c == '^' {
                    tachyon_count += 1;
                    if i > 0 {
                        new_tachyons.insert(i - 1);
                    }
                    if i < line.len() - 1 {
                        new_tachyons.insert(i + 1);
                    }
                } else {
                    new_tachyons.insert(i);
                }
            }
        }
        tachyons = new_tachyons;
    }

    println!("total: {}", tachyon_count);
}

fn part2(filename: &str) {
    let lines: Vec<String> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let width = lines[0].len();

    // At every line, the sum of each element in this
    let mut paths = vec![0_u64; width]; //).map(|_| 0_u64).collect();

    for line in &lines {
        let mut new_paths = vec![0_u64; width];
        for (i, c) in line.char_indices() {
            if c == 'S' {
                new_paths[i] = 1;
                break;
            }

            if paths[i] > 0 {
                if c == '^' {
                    if i > 0 {
                        new_paths[i - 1] += paths[i];
                    }
                    if i < line.len() - 1 {
                        new_paths[i + 1] += paths[i];
                    }
                } else {
                    new_paths[i] += paths[i];
                }
            }
        }
        paths = new_paths;
    }

    println!("total: {}", paths.iter().sum::<u64>());
}

pub fn day7(filename: &str) {
    part1(filename);
    part2(filename);
}
