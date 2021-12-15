use advent_of_rust_2021::input_read::read_to_2d_byte_array;
use itertools::Itertools;
use std::cmp::Ordering;
use std::io;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: usize,
    cost: usize,
}

// From: https://doc.rust-lang.org/std/collections/binary_heap/index.html
fn shortest_path(adj_list: &[Vec<Edge>], start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = std::collections::BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

fn main() {
    let input = read_to_2d_byte_array(io::stdin().lock());
    println!("Day 15, part 1: {}", part1(&input));
    println!("Day 15, part 2: {}", part2(&input));
}
fn create_graph(input: &[Vec<u8>]) -> Vec<Vec<Edge>> {
    let height = input.len() as i32;
    let width = input.first().unwrap().len() as i32;

    let insert = |row: i32, column: i32, edges: &mut Vec<Edge>| {
        if row < 0 || column < 0 || row >= height || column >= width {
            return;
        }

        edges.push(Edge {
            node: (row * width + column) as usize,
            cost: input[row as usize][column as usize] as usize,
        });
    };

    let mut graph = Vec::new();
    for row in 0..height {
        for column in 0..width {
            let mut edges: Vec<Edge> = Vec::new();
            insert(row - 1, column, &mut edges);
            insert(row, column - 1, &mut edges);
            insert(row, column + 1, &mut edges);
            insert(row + 1, column, &mut edges);
            graph.push(edges);
        }
    }

    graph
}

fn part1(input: &[Vec<u8>]) -> usize {
    let graph = create_graph(input);
    shortest_path(&graph, 0, input.len() * input.first().unwrap().len() - 1).unwrap()
}

fn extend_input(input: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut extended = Vec::with_capacity(input.len() * 5);

    let input = input
        .iter()
        .map(|row| {
            let mut new_row = Vec::new();
            for i in 0..5 {
                for cell in row {
                    let risk = cell + i;
                    new_row.push(if risk > 9 { risk - 9 } else { risk });
                }
            }
            new_row
        })
        .collect_vec();

    let create_extended = |risk_increase: u8| {
        input
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cave| {
                        let risk = cave + risk_increase;
                        if risk > 9 {
                            risk - 9
                        } else {
                            risk
                        }
                    })
                    .collect_vec()
            })
            .collect_vec()
    };

    extended.extend(input.clone());
    extended.extend(create_extended(1));
    extended.extend(create_extended(2));
    extended.extend(create_extended(3));
    extended.extend(create_extended(4));

    extended
}

fn part2(input: &[Vec<u8>]) -> usize {
    let extended_input = extend_input(input);
    let graph = create_graph(&extended_input);
    shortest_path(
        &graph,
        0,
        extended_input.len() * extended_input.first().unwrap().len() - 1,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let input =
            read_to_2d_byte_array(BufReader::new(File::open("inputs/day15/input").unwrap()));
        assert_eq!(part1(&input), 604);
        assert_eq!(part2(&input), 2907);
    }
}
