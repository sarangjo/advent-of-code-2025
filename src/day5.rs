use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn day5(filename: &str) {
    part1(filename);
    part2(filename);
}

#[derive(Debug)]
struct Ranges {
    ranges: Vec<(u64, u64)>,
}

impl Ranges {
    fn new() -> Self {
        Ranges { ranges: Vec::new() }
    }

    fn add_range(&mut self, new_range: (u64, u64)) {
        let mut i = 0;
        while i < self.ranges.len() {
            // Before this range, time to insert
            if new_range.0 < self.ranges[i].0 && new_range.1 < self.ranges[i].0 {
                self.ranges.insert(i, new_range);
                return;
            }
            if new_range.0 < self.ranges[i].0 && new_range.1 >= self.ranges[i].0 {
                // there's overlap, and the new range is lower
                self.ranges[i].0 = new_range.0;

                if new_range.1 <= self.ranges[i].1 {
                    // no changes to be made
                    return;
                }

                // could subsume more
                loop {
                    // ends before the next range
                    if i + 1 == self.ranges.len() || new_range.1 < self.ranges[i + 1].0 {
                        self.ranges[i].1 = new_range.1;
                        return;
                    }

                    if new_range.1 <= self.ranges[i + 1].1 {
                        // ends in i+1, delete i+1 and extend
                        let end_range = self.ranges.remove(i + 1);
                        self.ranges[i].1 = end_range.1;
                        return;
                    }

                    self.ranges.remove(i + 1);
                }
            }
            if new_range.0 >= self.ranges[i].0 && new_range.0 <= self.ranges[i].1 {
                if new_range.1 <= self.ranges[i].1 {
                    // fully subsumed
                    return;
                }

                // there's overlap
                loop {
                    if i + 1 == self.ranges.len() || new_range.1 < self.ranges[i + 1].0 {
                        // this range ends before i+1, so we just extend the current range and be done
                        self.ranges[i].1 = new_range.1;
                        return;
                    }
                    // this range ends inside or beyond i+1.

                    if new_range.1 <= self.ranges[i + 1].1 {
                        // ends in i+1, delete i+1 and extend
                        let end_range = self.ranges.remove(i + 1);
                        self.ranges[i].1 = end_range.1;
                        return;
                    }

                    self.ranges.remove(i + 1);
                }
            }
            i += 1
        }
        self.ranges.push(new_range);
    }
}

fn part1(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    // Get ranges
    let mut ranges = Ranges::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line == "" {
            // done with ranges
            break;
        }

        let mut parts = line.split('-');
        ranges.ranges.push((
            parts.next().unwrap().parse::<u64>().unwrap(),
            parts.next().unwrap().parse::<u64>().unwrap(),
        ));
    }

    let mut count = 0;
    loop {
        let Some(line_result) = lines.next() else {
            break;
        };

        let line = line_result.unwrap();
        let ingredient = line.parse::<u64>().unwrap();
        let mut found = false;
        for range in &ranges.ranges {
            if ingredient >= range.0 && ingredient <= range.1 {
                found = true;
                break;
            }
        }
        if found {
            count += 1;
        }
    }

    println!("total: {}", count);
}

fn part2(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    // Get ranges
    let mut ranges = Ranges::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line == "" {
            // done with ranges
            break;
        }

        let mut parts = line.split('-');
        ranges.add_range((
            parts.next().unwrap().parse::<u64>().unwrap(),
            parts.next().unwrap().parse::<u64>().unwrap(),
        ));
    }

    let total: u64 = ranges.ranges.iter().map(|r| r.1 - r.0 + 1).sum();

    println!("total: {}", total);
}
