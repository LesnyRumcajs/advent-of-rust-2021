use crate::Packet::{LiteralValue, Operator};
use std::io;
use std::io::BufRead;

fn read_input<R: BufRead>(reader: R) -> String {
    reader.lines().filter_map(Result::ok).next().unwrap()
}

fn main() {
    let packet = read_input(io::stdin().lock());
    println!("Day 16, part 1: {}", part1(&packet));
    println!("Day 16, part 2: {}", part2(&packet));
}

fn hex_to_binary(s: &str) -> String {
    s.chars().fold(String::new(), |bin, hex| {
        let num = u32::from_str_radix(&hex.to_string(), 16).unwrap();
        bin + &format!("{:04b}", num)
    })
}

enum Packet {
    LiteralValue(ValuePacket),
    Operator(OperatorPacket),
}

struct ValuePacket {
    version: u32,
    value: u64,
}

struct OperatorPacket {
    version: u32,
    type_id: u32,
    packets: Vec<Packet>,
}

fn read_packets(packet: &str, mut idx: usize) -> (Packet, usize) {
    let version = u32::from_str_radix(&packet[idx..idx + 3], 2).unwrap();
    idx += 3;
    let type_id = u32::from_str_radix(&packet[idx..idx + 3], 2).unwrap();
    idx += 3;

    let content = if type_id == 4 {
        let mut value = String::new();
        while &packet[idx..idx + 1] == "1" {
            value.push_str(&packet[idx + 1..idx + 5]);
            idx += 5;
        }
        value.push_str(&packet[idx + 1..idx + 5]);
        idx += 5;

        let value_packet = ValuePacket {
            version,
            value: u64::from_str_radix(&value, 2).unwrap(),
        };
        Packet::LiteralValue(value_packet)
    } else {
        let length_type_id = &packet[idx..idx + 1];
        idx += 1;

        let mut sub_packets = Vec::new();
        if length_type_id == "0" {
            let mut total_in_bits = usize::from_str_radix(&packet[idx..idx + 15], 2).unwrap();
            idx += 15;

            while total_in_bits > 0 {
                let (sub_packet, new_idx) = read_packets(packet, idx);
                sub_packets.push(sub_packet);
                total_in_bits -= new_idx - idx;
                idx = new_idx;
            }
        } else {
            let total_packets = usize::from_str_radix(&packet[idx..idx + 11], 2).unwrap();
            idx += 11;

            for _ in 0..total_packets {
                let (sub_packet, new_idx) = read_packets(packet, idx);
                sub_packets.push(sub_packet);
                idx = new_idx;
            }
        }
        let operator_packet = OperatorPacket {
            version,
            type_id,
            packets: sub_packets,
        };
        Packet::Operator(operator_packet)
    };

    (content, idx)
}

fn sum_versions(packet: &Packet) -> u32 {
    match packet {
        LiteralValue(val) => val.version,
        Packet::Operator(val) => {
            val.version
                + val
                    .packets
                    .iter()
                    .fold(0, |sum, packet| sum + sum_versions(packet))
        }
    }
}

fn calculate_packets(packet: &Packet) -> u64 {
    match packet {
        LiteralValue(val) => val.value,
        Operator(val) => match val.type_id {
            0 => val
                .packets
                .iter()
                .fold(0, |sum, packet| sum + calculate_packets(packet)),
            1 => val
                .packets
                .iter()
                .fold(1, |product, packet| product * calculate_packets(packet)),
            2 => val.packets.iter().map(calculate_packets).min().unwrap(),
            3 => val.packets.iter().map(calculate_packets).max().unwrap(),
            5 => {
                if calculate_packets(val.packets.first().unwrap())
                    > calculate_packets(val.packets.get(1).unwrap())
                {
                    1
                } else {
                    0
                }
            }
            6 => {
                if calculate_packets(val.packets.first().unwrap())
                    < calculate_packets(val.packets.get(1).unwrap())
                {
                    1
                } else {
                    0
                }
            }
            7 => {
                if calculate_packets(val.packets.first().unwrap())
                    == calculate_packets(val.packets.get(1).unwrap())
                {
                    1
                } else {
                    0
                }
            }
            _ => panic!("fiasco"),
        },
    }
}

fn part1(packet: &str) -> u32 {
    let stream = hex_to_binary(packet);
    let bits_packet = read_packets(&stream, 0);

    sum_versions(&bits_packet.0)
}
fn part2(packet: &str) -> u64 {
    let stream = hex_to_binary(packet);
    let bits_packet = read_packets(&stream, 0);

    calculate_packets(&bits_packet.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input = read_input(BufReader::new(File::open("inputs/day16/input").unwrap()));
        assert_eq!(part1(&input), 938);
        assert_eq!(part2(&input), 1495959086337);
    }
}
