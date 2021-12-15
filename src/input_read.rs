use std::io::BufRead;

pub fn read_to_2d_byte_array<R: BufRead>(reader: R) -> Vec<Vec<u8>> {
    reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().map(|ch| ch as u8 - b'0').collect())
        .collect()
}
