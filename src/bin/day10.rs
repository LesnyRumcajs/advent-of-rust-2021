use itertools::Itertools;
use std::io;
use std::io::BufRead;

fn read_input<R: BufRead>(reader: R) -> Vec<String> {
    reader.lines().filter_map(Result::ok).collect()
}

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 10, part 1: {}", part1(&input));
    println!("Day 10, part 2: {}", part2(&input));
}

fn score(ch: char) -> i32 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn match_and_score(ch1: char, ch2: char) -> (bool, i32) {
    (
        if ch1 == '(' {
            ch2 == ')'
        } else if ch1 == '[' {
            ch2 == ']'
        } else if ch1 == '{' {
            ch2 == '}'
        } else if ch1 == '<' {
            ch2 == '>'
        } else {
            false
        },
        score(ch2),
    )
}

fn score_stack(stack: &[char]) -> i64 {
    stack.iter().rev().fold(0, |acc, ch| {
        5 * acc
            + match ch {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("fiasco"),
            }
    })
}

fn part1(input: &[String]) -> i32 {
    input.iter().fold(0, |acc, line| {
        let mut stack = Vec::new();
        acc + line.chars().fold(0, |acc, ch| {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                _ => {
                    let prev = stack.pop();
                    if let Some(prev) = prev {
                        let (matched, score) = match_and_score(prev, ch);
                        if !matched {
                            return acc + score;
                        }
                    }
                }
            };
            acc
        })
    })
}
fn part2(input: &[String]) -> i64 {
    let scores = input
        .iter()
        .map(|line| {
            let mut stack = Vec::new();
            for ch in line.chars() {
                match ch {
                    '(' | '[' | '{' | '<' => stack.push(ch),
                    _ => {
                        let prev = stack.pop();
                        if let Some(prev) = prev {
                            let (matched, _) = match_and_score(prev, ch);
                            if !matched {
                                return None;
                            }
                        }
                    }
                };
            }
            Some(score_stack(&stack))
        })
        .flatten()
        .collect_vec();

    *scores.iter().sorted().nth(scores.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = read_input(BufReader::new(File::open("inputs/day10/input").unwrap()));
        assert_eq!(part1(&input), 321237);
        assert_eq!(part2(&input), 2360030859);
    }
}
