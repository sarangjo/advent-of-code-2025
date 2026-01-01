use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
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

    fn distance(self: &Self, other: &Point) -> f64 {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)) as f64)
            .sqrt()
    }
}

fn part1(filename: &str) {
    let points: Vec<Point> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| Point::from(&l.unwrap()))
        .collect();

    // Find the closest two points
    let mut best: (f64, (&Point, &Point)) = (-1_f64, (&Point(0, 0, 0), &Point(0, 0, 0)));
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = points[i].distance(&points[j]);
            if best.0 < 0_f64 || distance < best.0 {
                // new best
                best.0 = distance;
                best.1 = (&points[i], &points[j]);
            }
        }
    }

    println!("best {:?}, {:?}", best.1.0, best.1.1);
}

pub fn day8(filename: &str) {
    part1(filename);
}
