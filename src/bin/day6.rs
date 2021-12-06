use std::io;
use std::io::BufRead;

fn read_fish<R: BufRead>(reader: R) -> Vec<i32> {
    reader
        .lines()
        .filter_map(Result::ok)
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn main() {
    let fish = read_fish(io::stdin().lock());
    println!("Day 6, part 1: {}", part1(&fish));
    println!("Day 6, part 2: {}", part2(&fish));
}

fn solve(fish: &[i32], days: i32) -> u64 {
    let mut fish_groups = [0u64; 10];
    for fish in fish.iter() {
        fish_groups[*fish as usize] += 1;
    }

    for _day in 0..days {
        fish_groups[7] += fish_groups[0];
        fish_groups[9] += fish_groups[0];
        fish_groups[0] = 0;
        fish_groups.rotate_left(1);
    }

    fish_groups.iter().sum()
}

fn part1(fish: &[i32]) -> u64 {
    solve(fish, 80)
}

fn part2(fish: &[i32]) -> u64 {
    solve(fish, 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let fish = read_fish(BufReader::new(File::open("inputs/day6/input").unwrap()));
        assert_eq!(part1(&fish), 351188);
        assert_eq!(part2(&fish), 1595779846729);
    }
}
