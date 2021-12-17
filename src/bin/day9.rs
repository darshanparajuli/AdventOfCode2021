use aoc_2021::*;
use std::collections::{HashSet, VecDeque};

fn main() {
    let input = read_input_map(|s| {
        s.chars()
            .map(|e| e.to_digit(10).unwrap())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    })
    .into_boxed_slice();

    let height_map = HeightMap::from(input);
    part1(height_map.clone());
    part2(height_map);
}

fn part1(height_map: HeightMap) {
    let sum = height_map
        .find_low_points()
        .iter()
        .map(|(_, e)| e + 1)
        .sum::<u32>();
    print_part1_answer(sum);
}

fn part2(height_map: HeightMap) {
    let mut sizes = height_map
        .find_low_points()
        .into_iter()
        .map(|((x, y), p)| get_basin_size(&height_map, p, x, y))
        .collect::<Vec<_>>();
    sizes.sort();

    let answer = sizes.iter().rev().take(3).product::<usize>();
    print_part2_answer(answer);
}

fn get_basin_size(height_map: &HeightMap, point: u32, x: usize, y: usize) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(((x, y), point));

    while let Some(((x, y), p)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        let adjacent_points = height_map.get_adjacent_points(x as i32, y as i32);
        adjacent_points
            .into_iter()
            .filter_map(|e| e)
            .filter(|(_, ap)| *ap != 9 && *ap > p)
            .for_each(|((x1, y1), ap)| {
                queue.push_back(((x1, y1), ap));
            });
    }

    visited.len()
}

#[derive(Debug, Clone)]
struct HeightMap {
    map: Box<[Box<[u32]>]>,
    w: usize,
    h: usize,
}

impl HeightMap {
    fn from(map: Box<[Box<[u32]>]>) -> Self {
        let w = map.first().unwrap().len();
        let h = map.len();
        Self { map, w, h }
    }

    fn get(&self, x: i32, y: i32) -> Option<u32> {
        if y < 0 || y as usize >= self.h || x < 0 || x as usize >= self.w {
            return None;
        }

        Some(self.map[y as usize][x as usize])
    }

    fn get_adjacent_points(&self, x: i32, y: i32) -> [Option<((usize, usize), u32)>; 4] {
        let left = self
            .get(x - 1, y)
            .map(|e| ((x as usize - 1, y as usize), e));
        let right = self
            .get(x + 1, y)
            .map(|e| ((x as usize + 1, y as usize), e));
        let top = self
            .get(x, y - 1)
            .map(|e| ((x as usize, y as usize - 1), e));
        let bottom = self
            .get(x, y + 1)
            .map(|e| ((x as usize, y as usize + 1), e));
        [left, right, top, bottom]
    }

    fn find_low_points(&self) -> Vec<((usize, usize), u32)> {
        let mut v = vec![];
        for y in 0..self.h {
            for x in 0..self.w {
                let point = self.get(x as i32, y as i32).unwrap();
                let adjacent_points = self.get_adjacent_points(x as i32, y as i32);

                let is_low_point = adjacent_points
                    .iter()
                    .copied()
                    .filter_map(|e| e)
                    .all(|(_, e)| point < e);

                if is_low_point {
                    v.push(((x, y), point));
                }
            }
        }
        v
    }
}
