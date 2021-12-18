use aoc_2021::*;
use std::collections::VecDeque;

const ROWS: usize = 10;
const COLS: usize = 10;

fn main() {
    let input = parse_input();
    part1(input);
    part2(input);
}

fn part1(mut grid: Grid) {
    let count: u32 = (0..100).map(|_| grid.step()).sum();
    print_part1_answer(count);
}

fn part2(mut grid: Grid) {
    let mut step = 0;
    loop {
        let flashes = grid.step();
        step += 1;

        if flashes as usize == ROWS * COLS {
            break;
        }
    }

    print_part2_answer(step);
}

#[derive(Debug, Copy, Clone)]
struct Grid {
    octopuses: [[u32; COLS]; ROWS],
}

impl Grid {
    fn step(&mut self) -> u32 {
        for y in 0..ROWS {
            for x in 0..COLS {
                self.octopuses[y][x] += 1;
            }
        }

        let mut flashes = VecDeque::new();
        let mut flash_count = 0;

        let neighbors = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        loop {
            for y in 0..ROWS {
                for x in 0..COLS {
                    if self.octopuses[y][x] > 9 {
                        self.octopuses[y][x] = 0;
                        flashes.push_back((x, y));
                        flash_count += 1;
                    }
                }
            }

            if flashes.is_empty() {
                break flash_count;
            }

            while let Some((x, y)) = flashes.pop_front() {
                neighbors
                    .iter()
                    .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
                    .filter(|(x, y)| *x >= 0 && *x < COLS as i32 && *y >= 0 && *y < ROWS as i32)
                    .map(|(x, y)| (x as usize, y as usize))
                    .for_each(|(x, y)| {
                        if self.octopuses[y][x] != 0 {
                            self.octopuses[y][x] += 1;
                            if self.octopuses[y][x] > 9 {
                                self.octopuses[y][x] = 0;
                                flashes.push_back((x, y));
                                flash_count += 1;
                            }
                        }
                    });
            }
        }
    }
}

impl Grid {}

fn parse_input() -> Grid {
    let input = read_input_map(|s| {
        s.chars()
            .map(|e| e.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    });

    assert_eq!(input.len(), ROWS);

    let mut octopuses = [[0u32; COLS]; ROWS];

    for y in 0..ROWS {
        let row = &input[y];

        assert_eq!(row.len(), COLS);

        for x in 0..COLS {
            octopuses[y][x] = row[x];
        }
    }

    Grid { octopuses }
}
