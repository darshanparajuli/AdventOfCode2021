use aoc_2021::*;

fn main() {
    let input = parse_input();
    part1(input.clone());
    part2(input);
}

fn part1(input: Input) {
    let mut paper = input.paper;

    match input.instructions.first().copied().unwrap() {
        FoldInstruction::Up(y) => {
            paper.fold_up(y);
        }
        FoldInstruction::Left(x) => {
            paper.fold_left(x);
        }
    }

    let mut count = 0;
    for y in 0..paper.h {
        for x in 0..paper.w {
            if paper.get(x, y).unwrap() {
                count += 1;
            }
        }
    }

    print_part1_answer(count);
}

fn part2(input: Input) {
    let mut paper = input.paper;

    for i in input.instructions {
        match i {
            FoldInstruction::Up(y) => {
                paper.fold_up(y);
            }
            FoldInstruction::Left(x) => {
                paper.fold_left(x);
            }
        }
    }

    // Answer is on the print out.
    paper.print();
}

#[derive(Debug, Clone)]
struct Input {
    paper: Paper,
    instructions: Vec<FoldInstruction>,
}

#[derive(Debug, Clone)]
struct Paper {
    values: Box<[Box<[bool]>]>,
    w: usize,
    h: usize,
}

impl Paper {
    fn get(&self, x: usize, y: usize) -> Option<bool> {
        Some(self.values[y][x])
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.values[y][x] = value;
    }

    fn fold_up(&mut self, at: usize) {
        let mut y_up = at as i32 - 1;
        let mut y_down = at + 1;

        while y_up >= 0 && y_down < self.h {
            for x in 0..self.w {
                let value = self.get(x, y_up as usize).unwrap() || self.get(x, y_down).unwrap();
                self.set(x, y_up as usize, value);
            }
            y_up -= 1;
            y_down += 1;
        }

        self.h = at;
    }

    fn fold_left(&mut self, at: usize) {
        let mut x_left = at as i32 - 1;
        let mut x_right = at + 1;

        while x_left >= 0 && x_right < self.w {
            for y in 0..self.h {
                let value = self.get(x_left as usize, y).unwrap() || self.get(x_right, y).unwrap();
                self.set(x_left as usize, y, value);
            }
            x_left -= 1;
            x_right += 1;
        }

        self.w = at;
    }

    fn print(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                if self.get(x, y).unwrap() {
                    print!("# ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum FoldInstruction {
    Up(usize),
    Left(usize),
}

fn parse_input() -> Input {
    let input = read_input();
    let coords = input
        .iter()
        .take_while(|e| !e.is_empty())
        .map(|e| {
            let mut it = e.split(",");
            let x = it.next().unwrap().parse::<usize>().unwrap();
            let y = it.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let w = coords.iter().map(|(x, _)| x).copied().max().unwrap() as usize + 1;
    let h = coords.iter().map(|(_, y)| y).copied().max().unwrap() as usize + 1;

    let mut paper = Paper {
        values: vec![vec![false; w].into_boxed_slice(); h].into_boxed_slice(),
        w,
        h,
    };

    coords
        .iter()
        .copied()
        .for_each(|(x, y)| paper.set(x, y, true));

    let instructions = input
        .iter()
        .skip_while(|e| !e.starts_with("fold"))
        .map(|e| {
            let s = &e[11..];
            let mut it = s.split("=");
            let axis = it.next().unwrap();
            let value = it.next().unwrap().parse::<usize>().unwrap();
            match axis {
                "x" => FoldInstruction::Left(value),
                "y" => FoldInstruction::Up(value),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    Input {
        paper,
        instructions,
    }
}
