use itertools::Itertools;
use std::io;
use std::io::BufRead;

type SingleInput = (Vec<String>, Vec<String>);
type AllInput = Vec<SingleInput>;

fn read_input<R: BufRead>(reader: R) -> AllInput {
    reader
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            let (patterns, output) = l.split('|').collect_tuple().unwrap();
            (
                patterns.split_whitespace().map(|n| n.to_owned()).collect(),
                output.split_whitespace().map(|n| n.to_owned()).collect(),
            )
        })
        .collect()
}

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 8, part 1: {}", part1(&input));
    println!("Day 8, part 2: {}", part2(&input));
}

fn part1(input: &[SingleInput]) -> usize {
    input.iter().fold(0, |acc, input| {
        acc + input
            .1
            .iter()
            .filter(|&segments| [2, 3, 4, 7].contains(&segments.len()))
            .count()
    })
}
fn part2(input: &[SingleInput]) -> usize {
    input.iter().fold(0, |acc, input| {
        let mut candidates = vec![vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']; 7];

        // discard impossible positions
        for pattern in input.0.iter() {
            let mut retain = |segment_no: usize, input: &String| {
                candidates[segment_no].retain(|n| !input.chars().contains(n));
            };

            if pattern.len() == 2 {
                retain(0, pattern);
                retain(1, pattern);
                retain(3, pattern);
                retain(4, pattern);
                retain(6, pattern);
            }

            if pattern.len() == 3 {
                retain(1, pattern);
                retain(3, pattern);
                retain(4, pattern);
                retain(6, pattern);
            }

            if pattern.len() == 4 {
                retain(0, pattern);
                retain(4, pattern);
                retain(6, pattern);
            }
        }

        // some nasty heuristics
        let to_delete = candidates[6].clone();
        candidates[3].retain(|&n| !to_delete.contains(&n));
        let to_delete = candidates[0].clone();
        candidates[2].retain(|&n| !to_delete.contains(&n));

        // even more heuristics
        // given segment count is same across all inputs
        let mut retain_with_count = |segment_no: usize, count| {
            candidates[segment_no].retain(|&n| {
                input
                    .0
                    .iter()
                    .filter(|pattern| pattern.chars().contains(&n))
                    .count()
                    == count
            });
        };

        retain_with_count(3, 7);
        retain_with_count(1, 6);
        retain_with_count(2, 8);
        retain_with_count(5, 9);
        retain_with_count(4, 4);
        retain_with_count(6, 7);

        acc + input
            .1
            .iter()
            .fold(String::new(), |sum, digit| {
                sum + if digit.len() == 2 {
                    "1"
                } else if digit.len() == 3 {
                    "7"
                } else if digit.len() == 4 {
                    "4"
                } else if digit.len() == 7 {
                    "8"
                } else if digit.len() == 5 && digit.chars().contains(candidates[1].first().unwrap())
                {
                    "5"
                } else if digit.len() == 5 && digit.chars().contains(candidates[4].first().unwrap())
                {
                    "2"
                } else if digit.len() == 5 {
                    "3"
                } else if digit.len() == 6
                    && !digit.chars().contains(candidates[3].first().unwrap())
                {
                    "0"
                } else if digit.len() == 6 && digit.chars().contains(candidates[2].first().unwrap())
                {
                    "9"
                } else {
                    "6"
                }
            })
            .parse::<usize>()
            .unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = read_input(BufReader::new(File::open("inputs/day8/input").unwrap()));
        assert_eq!(part1(&input), 247);
        assert_eq!(part2(&input), 933305);
    }
}
