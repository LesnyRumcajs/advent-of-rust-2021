use std::io;
use std::io::BufRead;

fn read_diagnostics<R: BufRead>(reader: R) -> Vec<String> {
    reader.lines().filter_map(Result::ok).collect()
}

fn main() {
    let diagnostics = read_diagnostics(io::stdin().lock());
    println!("Day 3, part 1: {}", part1(&diagnostics));
    println!("Day 3, part 2: {}", part2(&diagnostics));
}

fn part1(diagnostics: &[String]) -> u32 {
    let width = diagnostics.first().unwrap().len();

    let mut bit_count: Vec<(usize, usize)> = vec![(0, 0); width];
    for diagnostic in diagnostics.iter() {
        for (i, bit) in diagnostic.chars().enumerate() {
            if bit == '0' {
                bit_count[i] = (bit_count[i].0 + 1, bit_count[i].1)
            } else {
                bit_count[i] = (bit_count[i].0, bit_count[i].1 + 1)
            }
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for bit in bit_count.iter() {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        if bit.1 > bit.0 {
            gamma_rate |= 1;
        } else {
            epsilon_rate |= 1;
        }
    }
    gamma_rate * epsilon_rate
}

fn part2(diagnostics: &[String]) -> u32 {
    let width = diagnostics.first().unwrap().len();

    let mut oxygen_ratings: Vec<_> = diagnostics.iter().collect();
    let mut co2_scrubber_ratings: Vec<_> = diagnostics.iter().collect();

    let bit_counter = |v: &Vec<&String>, i| {
        let mut count = (0, 0);
        for entry in v {
            if entry.chars().nth(i).unwrap() == '0' {
                count = (count.0 + 1, count.1)
            } else {
                count = (count.0, count.1 + 1)
            }
        }
        count
    };

    for i in 0..width {
        if oxygen_ratings.len() > 1 {
            let bit = bit_counter(&oxygen_ratings, i);
            let most_common = if bit.1 >= bit.0 { '1' } else { '0' };
            oxygen_ratings.retain(|rating| rating.chars().nth(i).unwrap() == most_common);
        }
        if co2_scrubber_ratings.len() > 1 {
            let bit = bit_counter(&co2_scrubber_ratings, i);
            let least_common = if bit.0 > bit.1 { '1' } else { '0' };
            co2_scrubber_ratings.retain(|rating| rating.chars().nth(i).unwrap() == least_common);
        }
    }

    u32::from_str_radix(oxygen_ratings.first().unwrap(), 2).unwrap()
        * u32::from_str_radix(co2_scrubber_ratings.first().unwrap(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let diagnostics =
            read_diagnostics(BufReader::new(File::open("inputs/day3/input").unwrap()));
        assert_eq!(part1(&diagnostics), 2743844);
        assert_eq!(part2(&diagnostics), 6677951);
    }
}
