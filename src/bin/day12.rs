use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn read_input<R: BufRead>(reader: R) -> Vec<(String, String)> {
    reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.split('-')
                .map(|s| s.to_owned())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 12, part 1: {}", part1(&input));
    println!("Day 12, part 2: {}", part2(&input));
}

fn chain_vec(vec: &[String], value: &str) -> Vec<String> {
    let mut new = vec.to_owned();
    new.push(value.to_owned());
    new
}

fn explore_twice(
    tunnels: &HashMap<String, Vec<String>>,
    path: Vec<String>,
    visited_twice: bool,
) -> Vec<Vec<String>> {
    let mut paths = Vec::new();

    for next in tunnels.get(path.last().unwrap()).unwrap() {
        let is_lowercase = next.chars().all(|ch| ch.is_lowercase());
        if next == "end" {
            paths.push(chain_vec(&path, next));
        } else if !is_lowercase || !path.contains(next) || !visited_twice && next != "start" {
            let visited_twice = visited_twice || is_lowercase && path.contains(next);
            for more_paths in explore_twice(tunnels, chain_vec(&path, next), visited_twice) {
                paths.push(more_paths);
            }
        }
    }
    paths
}

fn explore(tunnels: &HashMap<String, Vec<String>>, path: Vec<String>) -> Vec<Vec<String>> {
    let mut paths = Vec::new();

    for next in tunnels.get(path.last().unwrap()).unwrap() {
        if next == "end" {
            paths.push(chain_vec(&path, next));
        } else if !next.chars().any(|ch| ch.is_lowercase()) || !path.contains(next) {
            for more_paths in explore(tunnels, chain_vec(&path, next)) {
                paths.push(more_paths);
            }
        }
    }

    paths
}

fn input_to_map(input: &[(String, String)]) -> HashMap<String, Vec<String>> {
    let mut tunnels: HashMap<String, Vec<String>> = HashMap::new();

    for entry in input {
        tunnels
            .entry(entry.0.to_owned())
            .or_insert_with(Vec::new)
            .push(entry.1.to_owned());
        tunnels
            .entry(entry.1.to_owned())
            .or_insert_with(Vec::new)
            .push(entry.0.to_owned());
    }
    tunnels
}

fn part1(input: &[(String, String)]) -> usize {
    let tunnels = input_to_map(input);
    explore(&tunnels, vec!["start".to_owned()]).len()
}

fn part2(input: &[(String, String)]) -> usize {
    let tunnels = input_to_map(input);
    explore_twice(&tunnels, vec!["start".to_owned()], false).len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = read_input(BufReader::new(File::open("inputs/day12/input").unwrap()));
        assert_eq!(part1(&input), 5228);
        assert_eq!(part2(&input), 131228);
    }
}
