use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
struct Line((i32, i32), (i32, i32));

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks: ((i32, i32), (i32, i32)) = s
            .split(' ')
            .filter(|&l| l != "->")
            .map(|l| {
                l.split(',')
                    .map(|pos| pos.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();

        Ok(Self {
            0: chunks.0,
            1: chunks.1,
        })
    }
}

fn read_input<R: BufRead>(reader: R) -> Vec<Line> {
    reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.parse().unwrap())
        .collect()
}
fn main() {
    let lines = read_input(io::stdin().lock());
    println!("Day 5, part 1: {}", part1(&lines));
    println!("Day 5, part 2: {}", part2(&lines));
}

fn part1(lines: &[Line]) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    for line in lines {
        let min_x = line.0 .0.min(line.1 .0);
        let max_x = line.0 .0.max(line.1 .0);
        let min_y = line.0 .1.min(line.1 .1);
        let max_y = line.0 .1.max(line.1 .1);
        if min_x == max_x {
            for y in min_y..=max_y {
                *map.entry((min_x, y)).or_insert(0) += 1;
            }
        } else if min_y == max_y {
            for x in min_x..=max_x {
                *map.entry((x, min_y)).or_insert(0) += 1;
            }
        }
    }

    map.values().filter(|&&val| val > 1).count() as i32
}
fn part2(lines: &[Line]) -> i32 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    for line in lines {
        let min_x = line.0 .0.min(line.1 .0);
        let max_x = line.0 .0.max(line.1 .0);
        let min_y = line.0 .1.min(line.1 .1);
        let max_y = line.0 .1.max(line.1 .1);
        if min_x == max_x {
            for y in min_y..=max_y {
                *map.entry((min_x, y)).or_insert(0) += 1;
            }
        } else if min_y == max_y {
            for x in min_x..=max_x {
                *map.entry((x, min_y)).or_insert(0) += 1;
            }
        } else {
            for i in 0..=(max_x - min_x) {
                let (x, y) = if line.0 .0 < line.1 .0 && line.0 .1 < line.1 .1 {
                    (min_x + i, min_y + i)
                } else if line.0 .0 < line.1 .0 && line.0 .1 > line.1 .1 {
                    (min_x + i, max_y - i)
                } else if line.0 .0 > line.1 .0 && line.0 .1 < line.1 .1 {
                    (max_x - i, min_y + i)
                } else {
                    (max_x - i, max_y - i)
                };
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    map.values().filter(|&&val| val > 1).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let lines = read_input(BufReader::new(File::open("inputs/day5/input").unwrap()));
        assert_eq!(part1(&lines), 7085);
        assert_eq!(part2(&lines), 20271);
    }
}
