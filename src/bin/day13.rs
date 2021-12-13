use crate::Fold::{Horizontal, Vertical};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

enum Fold {
    Horizontal(i32),
    Vertical(i32),
}

fn read_input<R: BufRead>(reader: R) -> (Vec<Point>, Vec<Fold>) {
    let mut points = Vec::new();
    let mut folds = Vec::new();

    reader
        .lines()
        .filter_map(Result::ok)
        .for_each(|line: String| {
            if line.starts_with("fold") {
                let (direction, value) = line.split('=').collect_tuple().unwrap();
                let value = value.parse().unwrap();
                if direction == "fold along y" {
                    folds.push(Horizontal(value));
                } else {
                    folds.push(Vertical(value));
                }
            } else if !line.is_empty() {
                let (x, y) = line.split(',').collect_tuple().unwrap();
                points.push(Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                });
            }
        });

    (points, folds)
}

fn main() {
    let (points, folds) = read_input(io::stdin().lock());
    println!("Day 13, part 1: {}", part1(&points, &folds));
    println!("Day 13, part 2: {}", part2(&points, &folds));
}

type Sheet = HashSet<Point>;

fn fold_sheet(sheet: Sheet, fold: &Fold) -> Sheet {
    let mut folded_sheet: HashSet<Point> = HashSet::new();
    match fold {
        Horizontal(val) => {
            for point in sheet.iter() {
                match point.y.cmp(val) {
                    Ordering::Less => {
                        folded_sheet.insert(point.clone());
                    }
                    Ordering::Equal => {
                        panic!()
                    }
                    Ordering::Greater => {
                        folded_sheet.insert(Point {
                            y: *val - (point.y - *val).abs(),
                            x: point.x,
                        });
                    }
                }
            }
        }
        Vertical(val) => {
            for point in sheet.iter() {
                match point.x.cmp(val) {
                    Ordering::Less => {
                        folded_sheet.insert(point.clone());
                    }
                    Ordering::Equal => {
                        panic!()
                    }
                    Ordering::Greater => {
                        folded_sheet.insert(Point {
                            x: *val - (point.x - *val).abs(),
                            y: point.y,
                        });
                    }
                }
            }
        }
    }

    folded_sheet
}

fn part1(points: &[Point], folds: &[Fold]) -> usize {
    let sheet = HashSet::from_iter(points.iter().cloned());
    fold_sheet(sheet, folds.first().unwrap()).len()
}
fn part2(points: &[Point], folds: &[Fold]) -> usize {
    let sheet = HashSet::from_iter(points.iter().cloned());
    let sheet = folds.iter().fold(sheet, fold_sheet);

    let mut result = [[' '; 40]; 6];
    for point in sheet.iter() {
        result[point.y as usize][point.x as usize] = '#';
    }

    result.iter().for_each(|line| {
        line.iter().for_each(|ch| print!("{}", ch));
        println!();
    });

    sheet.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let (points, folds) = read_input(BufReader::new(File::open("inputs/day13/input").unwrap()));
        assert_eq!(part1(&points, &folds), 687);
        assert_eq!(part2(&points, &folds), 98);
    }
}
