use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, Hash)]
struct Point(i64, i64, i64);

impl Point {
    fn from(s: &str) -> Self {
        let mut parts = s.split(',');
        return Point(
            parts.next().unwrap().parse::<i64>().unwrap(),
            parts.next().unwrap().parse::<i64>().unwrap(),
            parts.next().unwrap().parse::<i64>().unwrap(),
        );
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for Point {}

#[derive(Debug, Clone, Copy)]
struct PointPair(Point, Point);

impl PointPair {
    fn distance(self: &Self) -> f64 {
        (((self.0.0 - self.1.0).pow(2)
            + (self.0.1 - self.1.1).pow(2)
            + (self.0.2 - self.1.2).pow(2)) as f64)
            .sqrt()
    }
}

impl Ord for PointPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance().total_cmp(&other.distance())
    }
}

impl PartialOrd for PointPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PointPair {
    fn eq(&self, other: &Self) -> bool {
        self.distance() == other.distance()
    }
}

impl Eq for PointPair {}

struct Playground {
    points: Vec<Point>,
    // TODO need Reverse?
    best_pairs: BinaryHeap<Reverse<PointPair>>,
    circuits: Vec<HashSet<Point>>,
}

impl Playground {
    fn new(filename: &str) -> Self {
        let mut playground = Self {
            points: BufReader::new(File::open(filename).unwrap())
                .lines()
                .map(|l| Point::from(&l.unwrap()))
                .collect(),
            best_pairs: BinaryHeap::new(),
            circuits: Vec::new(),
        };

        // Collect points by pairs
        for i in 0..playground.points.len() {
            for j in (i + 1)..playground.points.len() {
                playground.best_pairs.push(Reverse(PointPair(
                    playground.points[i],
                    playground.points[j],
                )));
            }
        }

        // Now we can start building circuits
        for p in &playground.points {
            playground.circuits.push(HashSet::from([p.clone()]));
        }

        playground
    }

    fn combine_best(self: &mut Self) -> PointPair {
        let best = self.best_pairs.pop().unwrap().0;

        let mut set_indices = [0_usize; 2];
        let mut idx = 0;
        for (i, c) in &mut self.circuits.iter().enumerate() {
            if c.contains(&best.0) {
                set_indices[idx] = i;
                idx += 1;
            }
            if c.contains(&best.1) {
                set_indices[idx] = i;
                idx += 1;
            }
            if idx == 2 {
                break;
            }
        }

        // Check if already in same circuit
        if set_indices[0] == set_indices[1] {
            return best;
        }

        // Remove the higher index
        let mut second_set = self.circuits.swap_remove(set_indices[1]);

        for p in second_set.drain() {
            self.circuits[set_indices[0]].insert(p);
        }

        return best;
    }
}

fn part1(filename: &str, is_sample: bool) {
    let mut pg = Playground::new(filename);

    let connection_count = if is_sample { 10 } else { 1000 };
    for _ in 0..connection_count {
        pg.combine_best();
    }

    // Final results
    pg.circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let result = pg.circuits[0].len() * pg.circuits[1].len() * pg.circuits[2].len();

    println!("result: {}", result);
}

fn part2(filename: &str) {
    let mut pg = Playground::new(filename);

    let mut best = pg.best_pairs.peek().unwrap().0;
    while pg.circuits.len() > 1 {
        best = pg.combine_best();
    }

    println!("result: {}", best.0.0 * best.1.0);
}

pub fn day8(filename: &str, is_sample: bool) {
    part1(filename, is_sample);
    part2(filename);
}
