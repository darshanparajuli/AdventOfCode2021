use aoc_2021::*;

fn main() {
    let input = read_input_map(|s| s.parse::<u32>().unwrap());
    part1(&input);
    part2(&input);
}

fn part1(input: &[u32]) {
    let count = input
        .windows(2)
        .map(|w| if w[1] > w[0] { 1 } else { 0 })
        .sum::<u32>();
    print_part1_answer(count);
}

fn part2(input: &[u32]) {
    let count = input
        .windows(4)
        .map(|w| {
            let a = w.iter().take(3).sum::<u32>();
            let b = w.iter().skip(1).sum::<u32>();
            if b > a {
                1
            } else {
                0
            }
        })
        .sum::<u32>();
    print_part2_answer(count);
}
