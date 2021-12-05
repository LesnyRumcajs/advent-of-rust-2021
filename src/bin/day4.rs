use std::io;
use std::io::BufRead;

#[derive(Debug, Default, PartialEq, Clone)]
struct Board {
    data: Vec<Vec<i32>>,
}

impl Board {
    fn calculate_score(&self, nums: &[i32]) -> i32 {
        self.data
            .iter()
            .flatten()
            .filter(|&num| !nums.contains(num))
            .sum::<i32>()
            * nums.last().unwrap()
    }

    fn is_winning(&self, nums: &[i32]) -> bool {
        // check horizontal
        for row in &self.data {
            if row.iter().all(|cell| nums.contains(cell)) {
                return true;
            }
        }

        // check vertical
        'outer: for column in 0..self.data.first().unwrap().len() {
            for row in 0..self.data.len() {
                if !nums.contains(&self.data[row][column]) {
                    continue 'outer;
                }
            }
            return true;
        }

        false
    }
}

fn read_input<R: BufRead>(reader: R) -> (Vec<i32>, Vec<Board>) {
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let numbers = lines
        .first()
        .unwrap()
        .split(',')
        .map(|num| num.parse())
        .filter_map(Result::ok)
        .collect();

    let mut board = Board::default();
    let mut boards = Vec::new();
    for line in &lines[2..] {
        if line.is_empty() {
            boards.push(board);
            board = Board::default();
        } else {
            let row = line
                .split_whitespace()
                .map(|num| num.parse())
                .filter_map(Result::ok)
                .collect();
            board.data.push(row);
        }
    }

    boards.push(board);

    (numbers, boards)
}

fn main() {
    let (numbers, boards) = read_input(io::stdin().lock());
    println!("Day 4, part 1: {}", part1(&numbers, &boards));
    println!("Day 4, part 2: {}", part2(&numbers, &boards));
}

fn part1(numbers: &[i32], boards: &[Board]) -> i32 {
    let mut nums_so_far = Vec::new();
    for num in numbers {
        nums_so_far.push(*num);
        for board in boards {
            if board.is_winning(&nums_so_far) {
                return board.calculate_score(&nums_so_far);
            }
        }
    }
    panic!("no solution!")
}
fn part2(numbers: &[i32], boards: &[Board]) -> i32 {
    let mut nums_so_far = Vec::new();
    let mut winning_boards = Vec::new();
    for draw in numbers {
        nums_so_far.push(*draw);
        for (i, board) in boards.iter().enumerate() {
            if board.is_winning(&nums_so_far) {
                if !winning_boards.contains(&i) {
                    winning_boards.push(i);
                }

                if winning_boards.len() == boards.len() {
                    return board.calculate_score(&nums_so_far);
                }
            }
        }
    }
    panic!("no solution!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let (numbers, boards) =
            read_input(BufReader::new(File::open("inputs/day4/input").unwrap()));
        assert_eq!(part1(&numbers, &boards), 6592);
        assert_eq!(part2(&numbers, &boards), 31755);
    }
}
