use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

#[derive(Debug, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn read_input<R: BufRead>(reader: R) -> TargetArea {
    let input = reader.lines().next().unwrap().unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-?\d+)").unwrap();
    }
    let matches: Vec<i32> = RE
        .find_iter(&input)
        .filter_map(|m| m.as_str().parse().ok())
        .collect();
    TargetArea {
        x_min: matches[0],
        x_max: matches[1],
        y_min: matches[2],
        y_max: matches[3],
    }
}

fn main() {
    let target_area = read_input(io::stdin().lock());
    println!("Day 17, part 1: {}", part1(&target_area));
    println!("Day 17, part 2: {}", part2(&target_area));
}

fn try_simulate(target_area: &TargetArea, mut vel_x: i32, mut vel_y: i32) -> Option<i32> {
    let mut pos = Point::default();
    let mut max_height = pos.y;

    while pos.x < target_area.x_max && pos.y > target_area.y_min {
        pos.x += vel_x;
        pos.y += vel_y;

        if pos.x >= target_area.x_min
            && pos.x <= target_area.x_max
            && pos.y >= target_area.y_min
            && pos.y <= target_area.y_max
        {
            return Some(max_height);
        }

        vel_x = 0.max(vel_x - 1);

        if vel_y == 0 {
            max_height = pos.y;
        }
        vel_y -= 1;
    }

    None
}

fn part1(target_area: &TargetArea) -> i32 {
    (1..=target_area.x_max)
        .cartesian_product(target_area.y_min..100)
        .filter_map(|(vel_x, vel_y)| try_simulate(target_area, vel_x, vel_y))
        .max()
        .unwrap()
}
fn part2(target_area: &TargetArea) -> usize {
    (1..=target_area.x_max)
        .cartesian_product(target_area.y_min..100)
        .filter(|&(vel_x, vel_y)| try_simulate(target_area, vel_x, vel_y).is_some())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let target_area = read_input(BufReader::new(File::open("inputs/day17/input").unwrap()));
        assert_eq!(part1(&target_area), 4095);
        assert_eq!(part2(&target_area), 3773);
    }
}
