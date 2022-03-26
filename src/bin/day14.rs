use aoc_2021::*;
use std::collections::HashMap;

fn main() {
    let input = parse_input();
    part1(input.clone());
    part2(input);
}

fn part1(input: Input) {
    println!("part 1: {}", solve(input, 10));
}

fn part2(input: Input) {
    println!("part 2: {}", solve(input, 40));
}

fn solve(input: Input, iter_count: u32) -> u64 {
    let template = input.template;
    let mut map = HashMap::new();

    for i in 0..template.len() - 1 {
        let pair = &template[i..i + 2];

        let (a, b) = &input.insertion_rules[pair];
        *map.entry(a.clone()).or_insert(0) += 1;
        *map.entry(b.clone()).or_insert(0) += 1;
    }

    for _ in 0..iter_count - 1 {
        let mut next_map = HashMap::new();

        for (k, v) in &map {
            let (a, b) = &input.insertion_rules[k];
            *next_map.entry(a.clone()).or_insert(0) += v;
            *next_map.entry(b.clone()).or_insert(0) += v;
        }

        map.clear();
        for (k, v) in next_map {
            map.insert(k, v);
        }
    }

    let mut map_count: HashMap<char, u64> = HashMap::new();
    for (k, v) in &map {
        k.chars().for_each(|c| {
            *map_count.entry(c).or_insert(0) += v;
        });
    }

    for v in map_count.values_mut() {
        *v = (*v as f64 / 2.0f64).ceil() as u64
    }

    let most = map_count.values().max().unwrap();
    let least = map_count.values().min().unwrap();

    most - least
}

#[derive(Debug, Clone)]
struct Input {
    template: String,
    insertion_rules: HashMap<String, (String, String)>,
}

fn parse_input() -> Input {
    let input = read_input();
    let template = input.first().unwrap().to_owned();

    let insertion_rules = input
        .iter()
        .skip(2)
        .map(|e| {
            let mut it = e.split(" -> ");
            let a = it.next().unwrap();
            let b = it.next().unwrap();

            (
                a.to_owned(),
                (
                    (a[0..1].to_owned() + &b).to_owned(),
                    (b.to_owned() + &a[1..2]).to_owned(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    Input {
        template,
        insertion_rules,
    }
}
