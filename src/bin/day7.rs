use aoc_2021::*;

fn main() {
    let input = read_input()
        .first()
        .unwrap()
        .split(",")
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    part1(input.clone());
    part2(input);
}

fn part1(input: Vec<i64>) {
    let answer = input.iter().fold(i64::MAX, |acc, pos| {
        input.iter().map(|e| (pos - e).abs()).sum::<i64>().min(acc)
    });
    print_part1_answer(answer);
}

fn part2(input: Vec<i64>) {
    let max = input.iter().max().copied().unwrap();
    let answer = (0..=max).into_iter().fold(i64::MAX, |acc, pos| {
        let cost = input
            .iter()
            .map(|e| {
                let steps = (pos - e).abs();
                (steps * (steps + 1)) / 2
            })
            .sum::<i64>();
        acc.min(cost)
    });

    print_part2_answer(answer);
}
