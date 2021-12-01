use itertools::Itertools;
use std::io::{self, BufRead};

fn read_numbers<R: BufRead>(reader: R) -> Vec<i32> {
    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|i| i.parse::<i32>().ok())
        .collect()
}

fn main() {
    let numbers = read_numbers(io::stdin().lock());
    println!("Day 1, part 1: {}", part1(&numbers));
    println!("Day 1, part 2: {}", part2(&numbers));
}

fn part1(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .tuple_windows()
        .fold(0, |sum, (a, b)| if b > a { sum + 1 } else { sum })
}

fn part2(numbers: &[i32]) -> i32 {
    numbers.windows(4).fold(0, |sum, window| {
        if window[1..].iter().sum::<i32>() > window[0..3].iter().sum::<i32>() {
            sum + 1
        } else {
            sum
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let numbers = read_numbers(BufReader::new(File::open("inputs/day1/input").unwrap()));
        assert_eq!(part1(&numbers), 1226);
        assert_eq!(part2(&numbers), 1252);
    }
}
