use itertools::Itertools;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, value) = s.split(' ').collect_tuple().ok_or(())?;
        let value = value.parse().map_err(|_| ())?;
        match instruction {
            "forward" => Ok(Instruction::Forward(value)),
            "down" => Ok(Instruction::Down(value)),
            "up" => Ok(Instruction::Up(value)),
            _ => panic!("fiasco"),
        }
    }
}

fn read_instructions<R: BufRead>(reader: R) -> Vec<Instruction> {
    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.parse::<Instruction>().ok())
        .collect()
}

fn main() {
    let instructions = read_instructions(io::stdin().lock());
    println!("Day 2, part 1: {}", part1(&instructions));
    println!("Day 2, part 2: {}", part2(&instructions));
}

fn part1(instructions: &[Instruction]) -> i32 {
    let (pos, depth) = instructions
        .iter()
        .fold((0, 0), |(pos, depth), instr| match instr {
            Instruction::Up(val) => (pos, depth - val),
            Instruction::Down(val) => (pos, depth + val),
            Instruction::Forward(val) => (pos + val, depth),
        });
    pos * depth
}
fn part2(instructions: &[Instruction]) -> i32 {
    let (pos, depth, _) =
        instructions
            .iter()
            .fold((0, 0, 0), |(pos, depth, aim), instr| match instr {
                Instruction::Up(val) => (pos, depth, aim - val),
                Instruction::Down(val) => (pos, depth, aim + val),
                Instruction::Forward(val) => (pos + val, aim * val + depth, aim),
            });
    pos * depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let instructions =
            read_instructions(BufReader::new(File::open("inputs/day2/input").unwrap()));
        assert_eq!(part1(&instructions), 1938402);
        assert_eq!(part2(&instructions), 1947878632);
    }
}
