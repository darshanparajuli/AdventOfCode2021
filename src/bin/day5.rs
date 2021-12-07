use aoc_2021::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    let input = read_input_map(|s| {
        let tokens = s.split(" -> ").collect::<Vec<_>>();
        assert_eq!(tokens.len(), 2);
        let to_points = |s: &str| {
            let result = s
                .split(",")
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            assert_eq!(result.len(), 2);
            Point::new(result[0], result[1])
        };

        let a = to_points(tokens[0]);
        let b = to_points(tokens[1]);
        Line::new(a, b)
    });

    part1(input.clone());
    part2(input);
}

fn part1(lines: Vec<Line>) {
    let lines = lines
        .iter()
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .copied()
        .collect::<Vec<_>>();

    let mut map = HashMap::new();

    for line in &lines {
        if line.is_vertical() {
            let mut start_y = if line.a.y > line.b.y {
                line.b.y
            } else {
                line.a.y
            };

            for _ in 0..=(line.a.y - line.b.y).abs() {
                match map.entry((line.a.x, start_y)) {
                    Entry::Occupied(mut o) => {
                        *o.get_mut() += 1;
                    }
                    Entry::Vacant(v) => {
                        v.insert(1);
                    }
                }

                start_y += 1;
            }
        } else {
            let mut start_x = if line.a.x > line.b.x {
                line.b.x
            } else {
                line.a.x
            };

            for _ in 0..=(line.a.x - line.b.x).abs() {
                match map.entry((start_x, line.a.y)) {
                    Entry::Occupied(mut o) => {
                        *o.get_mut() += 1;
                    }
                    Entry::Vacant(v) => {
                        v.insert(1);
                    }
                }

                start_x += 1;
            }
        }
    }

    let answer = map.values().copied().filter(|v| *v >= 2).count();
    print_part1_answer(answer);
}

fn part2(lines: Vec<Line>) {
    let mut map = HashMap::new();

    for line in &lines {
        if line.is_vertical() {
            let mut start_y = if line.a.y > line.b.y {
                line.b.y
            } else {
                line.a.y
            };

            for _ in 0..=(line.a.y - line.b.y).abs() {
                match map.entry((line.a.x, start_y)) {
                    Entry::Occupied(mut o) => {
                        *o.get_mut() += 1;
                    }
                    Entry::Vacant(v) => {
                        v.insert(1);
                    }
                }

                start_y += 1;
            }
        } else if line.is_horizontal() {
            let mut start_x = if line.a.x > line.b.x {
                line.b.x
            } else {
                line.a.x
            };

            for _ in 0..=(line.a.x - line.b.x).abs() {
                match map.entry((start_x, line.a.y)) {
                    Entry::Occupied(mut o) => {
                        *o.get_mut() += 1;
                    }
                    Entry::Vacant(v) => {
                        v.insert(1);
                    }
                }

                start_x += 1;
            }
        } else {
            let dx = (line.b.x - line.a.x).signum();
            let dy = (line.b.y - line.a.y).signum();

            let mut a = line.a;

            loop {
                match map.entry((a.x, a.y)) {
                    Entry::Occupied(mut o) => {
                        *o.get_mut() += 1;
                    }
                    Entry::Vacant(v) => {
                        v.insert(1);
                    }
                }

                if a == line.b {
                    break;
                }

                a.x += dx;
                a.y += dy;
            }
        }
    }

    let answer = map.values().copied().filter(|v| *v >= 2).count();
    print_part2_answer(answer);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }
}
