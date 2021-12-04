use aoc_2021::*;
use std::collections::{HashMap, HashSet};

const ROWS: usize = 5;
const COLUMNS: usize = 5;

fn main() {
    let input = parse_input();
    part1(input.clone());
    part2(input);
}

fn part1(mut input: InputData) {
    let mut answer = 0;
    'outer: for n in input.numbers.iter() {
        for board in &mut input.boards {
            board.mark(*n);

            if board.is_any_row_or_column_marked() {
                answer = n * board
                    .grid
                    .iter()
                    .filter(|c| !c.marked)
                    .map(|c| c.value)
                    .sum::<u32>();
                break 'outer;
            }
        }
    }

    print_part1_answer(answer);
}

fn part2(mut input: InputData) {
    let mut won = HashSet::new();
    let mut last_won = 0;
    let mut last_num = 0;
    for n in input.numbers.iter() {
        for (i, board) in input.boards.iter_mut().enumerate() {
            if won.contains(&i) {
                continue;
            }

            board.mark(*n);
            if board.is_any_row_or_column_marked() {
                last_won = i;
                last_num = *n;
                won.insert(i);
            }
        }
    }

    let answer = last_num
        * input.boards[last_won]
            .grid
            .iter()
            .filter(|c| !c.marked)
            .map(|c| c.value)
            .sum::<u32>();

    print_part2_answer(answer);
}

fn parse_input() -> InputData {
    let input = read_input()
        .iter()
        .filter(|e| !e.trim().is_empty())
        .cloned()
        .collect::<Vec<_>>();

    let numbers = input
        .first()
        .unwrap()
        .split(",")
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let boards = input[1..]
        .chunks(5)
        .map(|e| {
            let board_input = e
                .iter()
                .map(|s| {
                    s.trim()
                        .split(" ")
                        .filter(|e| !e.is_empty())
                        .map(|e| e.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            Board::new(board_input)
        })
        .collect::<Vec<_>>();

    InputData { numbers, boards }
}

#[derive(Clone)]
struct InputData {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Debug, Default, Copy, Clone)]
struct Cell {
    value: u32,
    marked: bool,
}

#[derive(Debug, Clone)]
struct Board {
    grid: [Cell; ROWS * COLUMNS],
    row_col_map: HashMap<u32, (usize, usize)>,
}

impl Board {
    fn new(input: Vec<Vec<u32>>) -> Self {
        let mut grid = [Cell::default(); 5 * 5];
        assert_eq!(input.len(), ROWS);
        let mut row_col_map = HashMap::new();

        for r in 0..ROWS {
            let row = &input[r];
            assert_eq!(row.len(), COLUMNS);
            for c in 0..COLUMNS {
                grid[r * COLUMNS + c].value = row[c];
                row_col_map.insert(row[c], (r, c));
            }
        }

        Self { grid, row_col_map }
    }

    fn is_any_row_or_column_marked(&self) -> bool {
        for r in 0..ROWS {
            let count = (0..COLUMNS)
                .map(|c| self.get_cell(r, c).marked)
                .filter(|e| *e)
                .count();

            if count == COLUMNS {
                return true;
            }
        }

        for c in 0..COLUMNS {
            let count = (0..ROWS)
                .map(|r| self.get_cell(r, c).marked)
                .filter(|e| *e)
                .count();

            if count == ROWS {
                return true;
            }
        }

        return false;
    }

    fn get_cell(&self, r: usize, c: usize) -> &Cell {
        &self.grid[r * COLUMNS + c]
    }

    fn get_cell_mut(&mut self, r: usize, c: usize) -> &mut Cell {
        &mut self.grid[r * COLUMNS + c]
    }

    fn mark(&mut self, num: u32) {
        if let Some((r, c)) = self.row_col_map.get(&num).copied() {
            self.get_cell_mut(r, c).marked = true;
        }
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                let cell = self.get_cell(r, c);
                if cell.marked {
                    print!("[{}] ", cell.value);
                } else {
                    print!("{} ", cell.value);
                }
            }
            println!();
        }
    }
}
