use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy)]
struct Point(u64, u64);

impl Point {
    fn from(s: &str) -> Self {
        let mut parts = s.split(',');
        return Point(
            parts.next().unwrap().parse::<u64>().unwrap(),
            parts.next().unwrap().parse::<u64>().unwrap(),
        );
    }
}

fn area(a: &Point, b: &Point) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

fn part1(filename: &str) {
    let points: Vec<Point> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| Point::from(&l.unwrap()))
        .collect();

    let mut pair = (Point(0, 0), Point(0, 0));
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            if area(&pair.0, &pair.1) < area(&points[i], &points[j]) {
                pair = (points[i], points[j])
            }
        }
    }

    println!("best area: {}", area(&pair.0, &pair.1));
}

pub fn day9(filename: &str) {
    part1(filename);
}
