use std::io;
use std::io::BufRead;

use itertools::Itertools;

fn read_input<R: BufRead>(reader: R) -> Vec<i64> {
    reader
        .lines()
        .find_map(Result::ok)
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn main() {
    let positions = read_input(io::stdin().lock());
    println!("Day 7, part 1: {}", part1(&positions));
    println!("Day 7, part 2: {}", part2(&positions));
}

fn part1(positions: &[i64]) -> i64 {
    let (min, max) = if let itertools::MinMaxResult::MinMax(min, max) = positions.iter().minmax() {
        (*min, *max)
    } else {
        panic!("input fiasco!");
    };

    (min..=max)
        .map(|level| {
            positions
                .iter()
                .fold(0, |sum, pos| sum + i64::abs(level - pos))
        })
        .min()
        .unwrap()
}
fn part2(positions: &[i64]) -> i64 {
    let (min, max) = if let itertools::MinMaxResult::MinMax(min, max) = positions.iter().minmax() {
        (*min, *max)
    } else {
        panic!("input fiasco!");
    };

    let sequence_sum = |distance| (2 + distance - 1) as f64 / 2.0 * distance as f64;

    (min..=max)
        .map(|level| {
            positions.iter().fold(0, |sum, pos| {
                sum + sequence_sum(i64::abs(level - pos)) as i64
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let positions = read_input(BufReader::new(File::open("inputs/day7/input").unwrap()));
        assert_eq!(part1(&positions), 342534);
        assert_eq!(part2(&positions), 94004208);
    }
}
