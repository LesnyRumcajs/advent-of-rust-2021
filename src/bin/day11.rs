use std::collections::HashSet;
use std::convert::TryInto;
use std::io;
use std::io::BufRead;

fn read_input<R: BufRead>(reader: R) -> Vec<Vec<u8>> {
    reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().map(|ch| ch as u8 - b'0').collect())
        .collect()
}

fn main() {
    let input = read_input(io::stdin().lock());
    println!("Day 11, part 1: {}", part1(&input));
    println!("Day 11, part 2: {}", part2(&input));
}

fn flash(flashed: &mut HashSet<(i32, i32)>, input: &mut Vec<Vec<u8>>, x: i32, y: i32) -> i32 {
    if x < 0
        || y < 0
        || x >= input.len().try_into().unwrap()
        || y >= input.len().try_into().unwrap()
    {
        return 0;
    }
    let mut flashes = 0;
    if input[x as usize][y as usize] > 9 {
        flashes += 1;
        flashed.insert((x, y));
        input[x as usize][y as usize] = 0;
        flashes += flash(flashed, input, x - 1, y - 1);
        flashes += flash(flashed, input, x - 1, y);
        flashes += flash(flashed, input, x - 1, y + 1);
        flashes += flash(flashed, input, x, y - 1);
        flashes += flash(flashed, input, x, y + 1);
        flashes += flash(flashed, input, x + 1, y - 1);
        flashes += flash(flashed, input, x + 1, y);
        flashes += flash(flashed, input, x + 1, y + 1);
    } else if !flashed.contains(&(x, y)) {
        input[x as usize][y as usize] += 1;
        if input[x as usize][y as usize] > 9 {
            flashes += flash(flashed, input, x, y);
        }
    }

    flashes
}

fn flash_all(input: &mut Vec<Vec<u8>>) -> (i32, bool) {
    let mut flashed: HashSet<(i32, i32)> = HashSet::new();
    let flashes = (0..input.len()).into_iter().fold(0, |acc, x| {
        acc + (0..input.len()).into_iter().fold(0, |acc, y| {
            acc + if input[x][y] > 9 {
                flash(
                    &mut flashed,
                    input,
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                )
            } else {
                0
            }
        })
    });

    (flashes, flashed.len() == input.len() * input.len())
}

fn part1(input: &[Vec<u8>]) -> i32 {
    let mut input = input.to_vec();

    (1..=100).into_iter().fold(0, |acc, _| {
        input
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));

        acc + flash_all(&mut input).0
    })
}
fn part2(input: &[Vec<u8>]) -> i32 {
    let mut input = input.to_vec();

    let mut step = 0;
    loop {
        step += 1;
        input
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|cell| *cell += 1));

        if flash_all(&mut input).1 {
            return step;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = read_input(BufReader::new(File::open("inputs/day11/input").unwrap()));
        assert_eq!(part1(&input), 1637);
        assert_eq!(part2(&input), 242);
    }
}
