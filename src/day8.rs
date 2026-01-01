use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, hash_set},
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug)]
struct PointPair<'a>(&'a Point, &'a Point);

impl PointPair<'_> {
    fn distance(self: &Self) -> f64 {
        (((self.0.0 - self.1.0).pow(2)
            + (self.0.1 - self.1.1).pow(2)
            + (self.0.2 - self.1.2).pow(2)) as f64)
            .sqrt()
    }
}

impl Ord for PointPair<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Backwards because we want reverse ordering for the max BinaryHeap
        // other.distance().total_cmp(&self.distance())
        self.distance().total_cmp(&other.distance())

        // if distance < 0_f64 {
        //     std::cmp::Ordering::Less
        // } else if distance > 0_f64 {
        //     std::cmp::Ord
        // }
    }
}

impl PartialOrd for PointPair<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PointPair<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.distance() == other.distance()
    }
}

impl Eq for PointPair<'_> {}

fn part1(filename: &str) {
    let points: Vec<Point> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| Point::from(&l.unwrap()))
        .collect();

    let mut best_pairs = BinaryHeap::new();

    // Collect points by pairs
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            best_pairs.push(Reverse(PointPair(&points[i], &points[j])));
        }
    }

    // Now we can start building circuits
    // TODO: start circuits with all points to begin with, and merge in the for loop instead of just
    // adding
    let mut circuits: Vec<HashSet<&Point>> = Vec::new();

    for _ in 0..3 {
        let best = best_pairs.pop().unwrap().0;
        println!("best {:?}", best);

        let mut inserted = false;
        for c in &mut circuits {
            if c.contains(best.0) || c.contains(best.1) {
                c.insert(best.0);
                c.insert(best.1);
                inserted = true;
                break;
            }
        }
        if !inserted {
            circuits.push(HashSet::from([best.0, best.1]));
        }

        println!("circuits: {:?}", circuits);
    }
}

pub fn day8(filename: &str) {
    part1(filename);
}
