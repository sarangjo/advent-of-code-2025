use std::fs;

pub fn day4(filename: &str) {
    part1(filename);
    part2(filename);
}

#[derive(Debug)]
struct Grid {
    chars: Vec<Vec<char>>,
}

impl Grid {
    fn from_str(s: &str) -> Self {
        Self {
            chars: s
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect(),
        }
    }

    fn width(&self) -> i32 {
        self.chars[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.chars.len() as i32
    }

    fn bump_count(&self, i: i32, j: i32) -> u32 {
        if self.chars[i as usize][j as usize] == '@' {
            1
        } else {
            0
        }
    }

    fn get(&self, i: i32, j: i32) -> char {
        self.chars[i as usize][j as usize]
    }

    fn neightbor_count(&self, i: i32, j: i32) -> u32 {
        let mut count = 0_u32;

        if i != 0 {
            if j != 0 {
                count += self.bump_count(i - 1, j - 1);
            }
            if j != self.width() - 1 {
                count += self.bump_count(i - 1, j + 1);
            }
            count += self.bump_count(i - 1, j);
        }
        if i != self.height() - 1 {
            if j != 0 {
                count += self.bump_count(i + 1, j - 1);
            }
            if j != self.width() - 1 {
                count += self.bump_count(i + 1, j + 1);
            }
            count += self.bump_count(i + 1, j);
        }
        if j != 0 {
            count += self.bump_count(i, j - 1);
        }
        if j != self.width() - 1 {
            count += self.bump_count(i, j + 1);
        }
        count
    }

    fn open_roll_count(&mut self) -> u32 {
        let mut total = 0_u32;

        let mut new_chars: Vec<Vec<char>> = Vec::with_capacity(self.height() as usize);

        for i in 0..self.height() {
            let mut row: Vec<char> = Vec::with_capacity(self.width() as usize);
            for j in 0..self.width() {
                if self.get(i, j) == '@' && self.neightbor_count(i, j) < 4 {
                    total += 1;
                    row.push('.');
                } else {
                    row.push(self.get(i, j));
                }
            }
            new_chars.push(row);
        }

        self.chars = new_chars;

        return total;
    }
}

fn part1(filename: &str) {
    let content = fs::read_to_string(filename).unwrap();

    let mut grid = Grid::from_str(&content);

    println!("total: {}", grid.open_roll_count());
}

fn part2(filename: &str) {
    let content = fs::read_to_string(filename).unwrap();
    let mut grid = Grid::from_str(&content);

    let mut total = 0_u32;

    loop {
        let cur_total = grid.open_roll_count();
        if cur_total == 0 {
            break;
        }
        total += cur_total;
    }

    println!("total: {}", total);
}
