use aoc_2021::*;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

fn main() {
    let input = read_input_map(|e| {
        e.chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<u32>>>()
            .unwrap()
    });
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: Vec<Vec<u32>>) {
    println!("part 1: {}", shortest_path_dist(&input));
}

fn part2(input: Vec<Vec<u32>>) {
    let y_len = input.len();
    let x_len = input[0].len();
    let mut new_map = input;
    new_map.resize(y_len * 5, vec![]);
    for row in new_map.iter_mut() {
        row.resize(x_len * 5, 0);
    }

    for y in 0..y_len {
        for x in x_len..new_map[y].len() {
            let mut value = new_map[y][x - x_len] + 1;
            if value > 9 {
                value = 1;
            }
            new_map[y][x] = value;
        }
    }

    for y in y_len..new_map.len() {
        for x in 0..new_map[y].len() {
            let mut value = new_map[y - y_len][x] + 1;
            if value > 9 {
                value = 1;
            }
            new_map[y][x] = value;
        }
    }

    println!("part 2: {}", shortest_path_dist(&new_map));
}

fn shortest_path_dist(input: &[Vec<u32>]) -> u32 {
    let mut dist = vec![];
    dist.resize(input.len(), vec![]);
    for i in 0..input.len() {
        dist[i].resize(input[i].len(), u32::MAX);
    }
    dist[0][0] = 0;

    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(Vertex::new()));

    while let Some(vertex) = min_heap.pop() {
        let vertex = vertex.0;
        let neighbors = [
            (vertex.x + 1, vertex.y),
            (vertex.x, vertex.y + 1),
            (vertex.x - 1, vertex.y),
            (vertex.x, vertex.y - 1),
        ];
        for (x, y) in neighbors.iter().copied().filter(|(x, y)| {
            y >= &0
                && y < &(input.len() as i32)
                && x >= &0
                && x < &(input[*y as usize].len() as i32)
        }) {
            let x = x as usize;
            let y = y as usize;
            let d = vertex.dist + input[y][x];
            if d < dist[y][x] {
                dist[y][x] = d;
                min_heap.push(Reverse(Vertex::with(x as i32, y as i32, d)));
            }
        }
    }

    return *dist.last().unwrap().last().unwrap();
}

#[derive(Eq, PartialEq, Hash)]
struct Vertex {
    x: i32,
    y: i32,
    dist: u32,
}

impl Vertex {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dist: 0,
        }
    }

    fn with(x: i32, y: i32, dist: u32) -> Self {
        Self { x, y, dist }
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.dist.cmp(&other.dist))
    }
}
