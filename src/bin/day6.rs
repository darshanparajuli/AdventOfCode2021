use aoc_2021::*;

fn main() {
    let input = read_input()
        .first()
        .unwrap()
        .split(",")
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    part1(input.clone());
    part2(input);
}

fn part1(mut input: Vec<u64>) {
    const NUM_DAYS: u32 = 80;

    for _ in 0..NUM_DAYS {
        let mut zero_count = 0;

        for i in &mut input {
            if *i == 0 {
                *i = 6;
                zero_count += 1;
            } else {
                *i -= 1;
            }
        }

        for _ in 0..zero_count {
            input.push(8);
        }
    }

    print_part1_answer(input.len());
}

fn part2(input: Vec<u64>) {
    const NUM_DAYS: u32 = 256;

    let mut input = input.iter().copied().map(|e| (e, 1)).collect::<Vec<_>>();
    let mut count = input.len();

    for _ in 0..NUM_DAYS {
        let mut zero_count = 0;

        for (i, c) in &mut input {
            if *i == 0 {
                *i = 6;
                zero_count += *c;
                count += *c;
            } else {
                *i -= 1;
            }
        }

        input.push((8, zero_count));
    }

    print_part2_answer(count);
}
