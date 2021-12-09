use itertools::Itertools;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn read_input<R: BufRead>(reader: R) -> Vec<Vec<u8>> {
    reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().map(|point| point as u8 - b'0').collect_vec())
        .collect_vec()
}

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 9, part 1: {}", part1(&input));
    println!("Day 9, part 2: {}", part2(&input));
}

fn part1(input: &[Vec<u8>]) -> i32 {
    input.iter().enumerate().fold(0, |acc, (x, row)| {
        acc + row
            .iter()
            .enumerate()
            .filter(|&(y, &point)| {
                !(x >= 1 && point >= input[x - 1][y]
                    || x < input.len() - 1 && point >= input[x + 1][y]
                    || y >= 1 && point >= input[x][y - 1]
                    || y < row.len() - 1 && point >= input[x][y + 1])
            })
            .fold(0, |acc, (_, point)| acc + *point as i32 + 1)
    })
}

fn flood_basin(
    visited: &mut HashSet<(usize, usize)>,
    area: &[Vec<u8>],
    x: usize,
    y: usize,
) -> usize {
    if !visited.insert((x, y)) || area[x][y] == 9 {
        return 0;
    }

    let mut sum = 1;
    if x >= 1 {
        sum += flood_basin(visited, area, x - 1, y);
    }

    if x < area.len() - 1 {
        sum += flood_basin(visited, area, x + 1, y);
    }

    if y >= 1 {
        sum += flood_basin(visited, area, x, y - 1);
    }

    if y < area.first().unwrap().len() - 1 {
        sum += flood_basin(visited, area, x, y + 1);
    }

    sum
}
fn part2(input: &[Vec<u8>]) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut basins: Vec<usize> = Vec::new();

    for (x, row) in input.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let basin = flood_basin(&mut visited, input, x, y);
            if basin > 0 {
                basins.push(basin);
            }
        }
    }
    basins.iter().sorted().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = read_input(BufReader::new(File::open("inputs/day9/input").unwrap()));
        assert_eq!(part1(&input), 594);
        assert_eq!(part2(&input), 858494);
    }
}
