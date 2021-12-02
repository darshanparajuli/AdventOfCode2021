use aoc_2021::*;

fn main() {
    let input = read_input_map(|s| {
        let tokens = s.split(" ").map(|e| e).collect::<Vec<_>>();
        let value = tokens[1].parse::<i32>().unwrap();
        match tokens[0] {
            "forward" => Command::Forward(value),
            "down" => Command::Down(value),
            "up" => Command::Up(value),
            _ => unreachable!(),
        }
    });

    part1(&input);
    part2(&input);
}

fn part1(input: &[Command]) {
    let mut horizontal_pos = 0;
    let mut depth = 0i32;

    for cmd in input {
        match cmd {
            Command::Up(value) => depth -= value,
            Command::Down(value) => depth += value,
            Command::Forward(value) => horizontal_pos += value,
        }
    }

    print_part1_answer(horizontal_pos * depth);
}

fn part2(input: &[Command]) {
    let mut horizontal_pos = 0;
    let mut depth = 0i32;
    let mut aim = 0i32;

    for cmd in input {
        match cmd {
            Command::Up(value) => aim -= value,
            Command::Down(value) => aim += value,
            Command::Forward(value) => {
                horizontal_pos += value;
                depth += aim * value;
            }
        }
    }

    print_part2_answer(horizontal_pos * depth);
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}
