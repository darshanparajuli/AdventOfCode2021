use aoc_2021::*;

fn main() {
    let input = read_input_map(|s| s.chars().collect::<Vec<_>>());
    part1(input.clone());
    part2(input);
}

fn part1(input: Vec<Vec<char>>) {
    let sum: u32 = input
        .iter()
        .map(|line| first_illegal_char(line))
        .filter_map(|e| e)
        .map(|e| syntax_error_scope(e))
        .sum();
    print_part1_answer(sum);
}

fn part2(mut input: Vec<Vec<char>>) {
    input.retain(|line| first_illegal_char(line).is_none());

    let mut scores = vec![];
    for i in &input {
        let mut score = 0u64;
        for c in closing_chars(i) {
            let points = match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            };

            score *= 5;
            score += points;
        }
        scores.push(score);
    }

    scores.sort();

    let answer = scores[scores.len() / 2];

    print_part2_answer(answer);
}

fn first_illegal_char(line: &[char]) -> Option<char> {
    let mut stack = vec![];

    for c in line {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
            }
            _ => {
                if let Some(open) = stack.pop() {
                    let expected_closing_char = closing_char_of(*open);
                    if expected_closing_char != *c {
                        return Some(*c);
                    }
                }
            }
        }
    }

    None
}

fn closing_chars(line: &[char]) -> Vec<char> {
    let mut stack = vec![];

    for c in line {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c.clone());
            }
            _ => {
                stack.pop();
            }
        }
    }

    for c in &mut stack {
        *c = closing_char_of(*c);
    }

    stack.reverse();
    stack
}

fn syntax_error_scope(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn closing_char_of(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}
