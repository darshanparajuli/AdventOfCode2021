use aoc_2021::*;
use std::collections::HashSet;

fn main() {
    let input = parse_input(read_input().first().unwrap().clone());
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: TargetArea) {
    let mut max_y = 0;
    // Trial and error :)
    let max_steps = 1000;
    for y in 0..1000 {
        for x in 0..20 {
            let result = launch(&input, x, y, max_steps);
            if let Some(y) = result {
                max_y = max_y.max(y);
            }
        }
    }
    println!("part 1: {}", max_y);
}

fn part2(input: TargetArea) {
    let mut set = HashSet::new();
    // Trial and error :)
    let max_steps = 1000;
    for y in -1000..1000 {
        for x in -500..500 {
            if is_target_hit(&input, x, y, max_steps) {
                set.insert((x, y));
            }
        }
    }
    println!("part 2: {}", set.len());
}

fn is_target_hit(target_area: &TargetArea, vx: i64, vy: i64, steps: u32) -> bool {
    launch(target_area, vx, vy, steps).is_some()
}

fn launch(target_area: &TargetArea, vx: i64, vy: i64, steps: u32) -> Option<i64> {
    let mut probe = Probe::new(vx, vy);

    let mut max_y = 0;

    for _ in 0..steps {
        probe.step();
        max_y = max_y.max(probe.y);

        if target_area.contains(&probe) {
            return Some(max_y);
        }
    }

    None
}

fn parse_input(input: String) -> TargetArea {
    let i = input.find('x').unwrap();
    let j = input.find(',').unwrap();

    let x_data = &input[i + 2..j];

    let i = x_data.find("..").unwrap();
    let min_x = x_data[..i].parse::<i64>().unwrap();
    let max_x = x_data[i + 2..].parse::<i64>().unwrap();

    let i = input.find('y').unwrap();
    let y_data = &input[i + 2..];

    let i = y_data.find("..").unwrap();
    let min_y = y_data[..i].parse::<i64>().unwrap();
    let max_y = y_data[i + 2..].parse::<i64>().unwrap();

    TargetArea {
        min_x,
        max_x,
        min_y,
        max_y,
    }
}

#[derive(Debug, Copy, Clone)]
struct TargetArea {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl TargetArea {
    fn contains(&self, p: &Probe) -> bool {
        p.x >= self.min_x && p.x <= self.max_x && p.y >= self.min_y && p.y <= self.max_y
    }
}

#[derive(Debug, Copy, Clone)]
struct Probe {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Probe {
    fn new(vx: i64, vy: i64) -> Self {
        Self { x: 0, y: 0, vx, vy }
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        if self.vx < 0 {
            self.vx += 1;
        } else if self.vx > 0 {
            self.vx -= 1;
        }

        self.vy -= 1;
    }
}
