use aoc_2021::*;

fn main() {
    let input = parse_snailfish_numbers(read_input());
    part1(input.clone());
    part2(input.clone());
}

fn part1(sf_numbers: Vec<Vec<TokenType>>) {
    let sum = sf_numbers
        .into_iter()
        .reduce(|acc, item| {
            let mut sum = add(&acc, &item);
            reduce(&mut sum);
            sum
        })
        .unwrap();
    println!("part 1: {}", magnitude(&sum));
}

fn part2(sf_numbers: Vec<Vec<TokenType>>) {
    let mut sums = vec![];

    for i in 0..sf_numbers.len() {
        for j in 0..sf_numbers.len() {
            if i == j {
                break;
            }

            {
                let mut sum = add(&sf_numbers[i], &sf_numbers[j]);
                reduce(&mut sum);
                sums.push(magnitude(&sum));
            }

            {
                let mut sum = add(&sf_numbers[j], &sf_numbers[i]);
                reduce(&mut sum);
                sums.push(magnitude(&sum));
            }
        }
    }

    println!("part 2: {}", sums.iter().max().unwrap());
}

fn add(a: &[TokenType], b: &[TokenType]) -> Vec<TokenType> {
    let mut result = vec![];
    result.push(TokenType::Open);
    result.extend(a);
    result.extend(b);
    result.push(TokenType::Close);
    result
}

fn reduce(sf_number: &mut Vec<TokenType>) {
    while reduce_once(sf_number) {
        // Reduce until can't be reduced.
    }
}

fn magnitude(sf_number: &[TokenType]) -> u64 {
    let mut stack = vec![];
    let mut index = 0;

    while index < sf_number.len() {
        match sf_number[index] {
            TokenType::Open => {}
            TokenType::Num(n) => {
                stack.push(n);
            }
            TokenType::Close => {
                let y = stack.pop().unwrap();
                let x = stack.pop().unwrap();
                stack.push(3 * x + 2 * y);
            }
        }
        index += 1;
    }

    stack.first().copied().unwrap()
}

struct SplitData {
    index: usize,
    n: u64,
}

struct ExplodeData {
    index: usize,
    x: u64,
    y: u64,
}

fn reduce_once(sf_number: &mut Vec<TokenType>) -> bool {
    if let Some(data) = can_explode(sf_number) {
        explode(data, sf_number);
        return true;
    }

    if let Some(data) = can_split(sf_number) {
        split(data, sf_number);
        return true;
    }

    false
}

fn can_explode(sf_number: &[TokenType]) -> Option<ExplodeData> {
    let mut depth = 0;
    let mut index = 0;
    while index < sf_number.len() {
        match sf_number[index] {
            TokenType::Open => {
                depth += 1;
                index += 1;
            }
            TokenType::Num(n) => {
                if depth == 5 {
                    // explode
                    if let TokenType::Num(y) = sf_number[index + 1] {
                        return Some(ExplodeData {
                            index: index - 1,
                            x: n,
                            y,
                        });
                    } else {
                        index += 1;
                    };
                    // break;
                } else {
                    index += 1;
                }
            }
            TokenType::Close => {
                depth -= 1;
                index += 1;
            }
        }
    }
    None
}

fn can_split(sf_number: &[TokenType]) -> Option<SplitData> {
    let mut index = 0;
    while index < sf_number.len() {
        match sf_number[index] {
            TokenType::Open => {
                index += 1;
            }
            TokenType::Num(n) => {
                if n >= 10 {
                    return Some(SplitData { index, n });
                } else {
                    index += 1;
                }
            }
            TokenType::Close => {
                index += 1;
            }
        }
    }
    None
}

fn explode(data: ExplodeData, sf_number: &mut Vec<TokenType>) {
    for _ in 0..4 {
        sf_number.remove(data.index);
    }
    sf_number.insert(data.index, TokenType::Num(0));

    for i in (0..data.index).rev() {
        match sf_number[i] {
            TokenType::Num(ref mut n) => {
                *n += data.x;
                break;
            }
            _ => {}
        }
    }

    for i in data.index + 1..sf_number.len() {
        match sf_number[i] {
            TokenType::Num(ref mut n) => {
                *n += data.y;
                break;
            }
            _ => {}
        }
    }
}

fn split(data: SplitData, sf_number: &mut Vec<TokenType>) {
    let x = (data.n as f64 / 2.0).floor() as u64;
    let y = (data.n as f64 / 2.0).ceil() as u64;
    sf_number.remove(data.index);
    sf_number.insert(data.index, TokenType::Close);
    sf_number.insert(data.index, TokenType::Num(y));
    sf_number.insert(data.index, TokenType::Num(x));
    sf_number.insert(data.index, TokenType::Open);
}

#[derive(Debug, Copy, Clone)]
enum TokenType {
    Open,
    Num(u64),
    Close,
}

fn parse_snailfish_numbers(input: Vec<String>) -> Vec<Vec<TokenType>> {
    input
        .into_iter()
        .map(|e| parse_snailfish_number(e))
        .collect()
}

fn parse_snailfish_number(input: String) -> Vec<TokenType> {
    let mut tokens = vec![];

    let mut index = 0;
    while index < input.len() {
        let c = input.chars().nth(index).unwrap();
        match c {
            '[' => {
                tokens.push(TokenType::Open);
                index += 1;
            }
            '0'..='9' => {
                let prev = input.chars().nth(index - 1).unwrap();
                let (num, next_index) = if prev == '[' {
                    let comma_index = index + input[index..].find(",").unwrap();
                    let num = input[index..comma_index].parse::<u64>().unwrap();
                    (num, comma_index)
                } else {
                    assert!(prev == ',');
                    let bracket_index = index + input[index..].find("]").unwrap();
                    let num = input[index..bracket_index].parse::<u64>().unwrap();
                    (num, bracket_index)
                };
                tokens.push(TokenType::Num(num));
                index = next_index;
            }
            ',' => {
                index += 1;
            }
            ']' => {
                tokens.push(TokenType::Close);
                index += 1;
            }
            _ => {
                index += 1;
            }
        }
    }

    tokens
}
