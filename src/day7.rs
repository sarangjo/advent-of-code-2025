use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn day7(filename: &str) {
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
