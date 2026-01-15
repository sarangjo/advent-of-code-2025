use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

#[derive(Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Point { x, y }
    }

    fn from_str(s: &str) -> Self {
        let mut parts = s.split(',');
        Point {
            x: parts.next().unwrap().parse::<u64>().unwrap(),
            y: parts.next().unwrap().parse::<u64>().unwrap(),
        }
    }
}

#[derive(Clone, Copy)]
struct Rect(Point, Point);

impl Rect {
    fn area(self: &Self) -> u64 {
        (self.0.x.abs_diff(self.1.x) + 1) * (self.0.y.abs_diff(self.1.y) + 1)
    }

    fn intersect(self: &Self, other: &Self) -> Option<Self> {
        let mut x_int: Option<(u64, u64)> = None;
        let mut y_int: Option<(u64, u64)> = None;

        // Find X intersection
        if other.0.x < self.0.x {
            if other.1.x < self.0.x {
                // no X intersection
                return None;
            }
            if other.1.x <= self.1.x {
                // X intersection
                x_int = Some((self.0.x, other.1.x));
            } else {
                x_int = Some((self.0.x, self.1.x));
            }
        } else if other.0.x < self.1.x {
            if other.1.x <= self.1.x {
                x_int = Some((other.0.x, other.1.x));
            } else {
                x_int = Some((other.0.x, self.1.x));
            }
        } else {
            return None;
        }

        // Find Y intersection
        if other.0.y < self.0.y {
            if other.1.y < self.0.y {
                // no X intersection
                return None;
            }
            if other.1.y <= self.1.y {
                // Y intersection
                y_int = Some((self.0.y, other.1.y));
            } else {
                y_int = Some((self.0.y, self.1.y));
            }
        } else if other.0.y < self.1.y {
            if other.1.y <= self.1.y {
                y_int = Some((other.0.y, other.1.y));
            } else {
                y_int = Some((other.0.y, self.1.y));
            }
        } else {
            return None;
        }

        return Some(Rect(
            Point::new(x_int.unwrap().0, y_int.unwrap().0),
            Point::new(x_int.unwrap().1, y_int.unwrap().1),
        ));
    }
}

fn part1(filename: &str) {
    let points: Vec<Point> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| Point::from_str(&l.unwrap()))
        .collect();

    let mut pair_opt: Option<Rect> = None;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let possible = Rect(points[i], points[j]);

            let Some(best_pair) = pair_opt else {
                pair_opt = Some(possible);
                continue;
            };

            if best_pair.area() < possible.area() {
                pair_opt = Some(possible)
            }
        }
    }

    println!("best area: {}", pair_opt.unwrap().area());
}

fn get_partitions(p1: &Point, p2: &Point) -> Vec<Rect> {
    if p1.y == p2.y {
        // horizontal line
        vec![
            Rect(
                Point::new(min(p1.x, p2.x), 0),
                Point::new(max(p1.x, p2.x), p2.y),
            ),
            Rect(
                Point::new(min(p1.x, p2.x), p2.y),
                Point::new(max(p1.x, p2.x), std::u64::MAX),
            ),
        ]
    } else {
        // vertical line
        vec![
            Rect(
                Point::new(0, min(p1.y, p2.y)),
                Point::new(p2.x, max(p1.y, p2.y)),
            ),
            Rect(
                Point::new(p1.x, min(p1.y, p2.y)),
                Point::new(std::u64::MAX, max(p1.y, p2.y)),
            ),
        ]
    }
}

fn choose_best_intersection(partitions1: &Vec<Rect>, partitions2: &Vec<Rect>) -> Vec<Rect> {
    let mut best_intersection_opt: Option<Rect> = None;

    for p1 in partitions1 {
        for p2 in partitions2 {
            let intersection_opt = p1.intersect(p2);

            // Check if there's any intersection at all
            let Some(intersection) = intersection_opt else {
                continue;
            };

            let Some(best_intersection) = best_intersection_opt else {
                best_intersection_opt = intersection_opt;
                continue;
            };

            // Compare
            if intersection.area() > best_intersection.area() {
                best_intersection_opt = Some(intersection);
            }
        }
    }
    return vec![];
}

fn part2(filename: &str) {
    // We iterate through pairs, but only consider "valid" rectangles. How can we tell, given a pair
    // of points, whether they are valid or not? Now instead of consuming independent points, we
    // need to slurp them into some sort of a structure that keeps track of the "inside" and
    // "outside" of our points.
    //
    // As each point is appended, we have a notion of the new division within our space.
    let mut lines = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap());

    // The first angle is interesting.
    let first_point = Point::from_str(&lines.next().unwrap());
    let second_point = Point::from_str(&lines.next().unwrap());

    let starting_possible_partitions = get_partitions(&first_point, &second_point);

    // Now the next pair determines which way we go
    let third_point = Point::from_str(&lines.next().unwrap());

    let interesecting_partitions = get_partitions(&second_point, &third_point);

    // Big decision: which two are the best ones to choose?
    let starting_partitions =
        choose_best_intersection(&starting_possible_partitions, &interesecting_partitions);

    let mut last_point_opt: Option<Point> = None;
    for l in lines {
        let cur_point = Point::from_str(&l);

        let Some(last_point) = last_point_opt else {
            last_point_opt = Some(cur_point);
            continue;
        };

        // We have two points. What are the partitions?

        // Which partition do we like?
    }
}

pub fn day9(filename: &str) {
    // part1(filename);
    part2(filename);
}
