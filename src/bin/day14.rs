use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn read_input<R: BufRead>(reader: R) -> (String, HashMap<(char, char), char>) {
    let mut template = String::new();
    let mut rules = HashMap::new();

    for line in reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.is_empty())
    {
        if template.is_empty() {
            template = line;
        } else {
            let (k, _, v) = line.split(' ').collect_tuple().unwrap();
            rules.insert(
                k.chars().collect_tuple().unwrap(),
                v.chars().next().unwrap(),
            );
        }
    }

    (template, rules)
}

fn main() {
    let (template, rules) = read_input(io::stdin().lock());
    println!("Day 14, part 1: {}", part1(&template, &rules));
    println!("Day 14, part 2: {}", part2(&template, &rules));
}

fn solve(template: &str, rules: &HashMap<(char, char), char>, steps: usize) -> usize {
    let mut polychunks: HashMap<(char, char), usize> = HashMap::new();

    for chunk in template.chars().tuple_windows() {
        *polychunks.entry(chunk).or_insert(0) += 1;
    }

    let mut counts: HashMap<char, usize> = template.chars().counts();
    for _ in 0..steps {
        let mut new_polychunks: HashMap<(char, char), usize> = HashMap::new();
        for (chunk, count) in polychunks {
            if let Some(new_chunk) = rules.get(&chunk) {
                *new_polychunks.entry((chunk.0, *new_chunk)).or_insert(0) += count;
                *new_polychunks.entry((*new_chunk, chunk.1)).or_insert(0) += count;
                *counts.entry(*new_chunk).or_insert(0) += count;
            } else {
                *new_polychunks.entry(chunk).or_insert(0) += count;
            }
        }

        polychunks = new_polychunks;
    }

    let counts = counts
        .iter()
        .sorted_by(|&kv1, &kv2| kv1.1.cmp(kv2.1))
        .map(|(_, v)| v)
        .collect_vec();
    **counts.last().unwrap() - **counts.first().unwrap()
}

fn part1(template: &str, rules: &HashMap<(char, char), char>) -> usize {
    solve(template, rules, 10)
}
fn part2(template: &str, rules: &HashMap<(char, char), char>) -> usize {
    solve(template, rules, 40)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let (template, rules) =
            read_input(BufReader::new(File::open("inputs/day14/input").unwrap()));
        assert_eq!(part1(&template, &rules), 2797);
        assert_eq!(part2(&template, &rules), 2926813379532);
    }
}
