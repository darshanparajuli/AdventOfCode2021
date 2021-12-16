use aoc_2021::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input_map(|s| {
        let entry = s.split(" | ").collect::<Vec<_>>();
        Entry {
            unique_patterns: entry[0]
                .split(" ")
                .map(|e| {
                    let mut v = e.chars().collect::<Vec<char>>();
                    v.sort();
                    v.into_iter().collect()
                })
                .collect(),
            output: entry[1]
                .split(" ")
                .map(|e| {
                    let mut v = e.chars().collect::<Vec<char>>();
                    v.sort();
                    v.into_iter().collect()
                })
                .collect(),
        }
    });
    part1(input.clone());
    part2(input);
}

fn part1(input: Vec<Entry>) {
    let mut count = 0;

    for entry in input {
        let unique_patterns = entry
            .unique_patterns
            .iter()
            .map(|e| e.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        let output = entry
            .output
            .iter()
            .map(|e| e.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        for p in unique_patterns {
            let c = output.iter().filter(|e| p == **e).count();
            match p.len() {
                2 | 4 | 3 | 7 => {
                    count += c;
                }
                _ => {}
            }
        }
    }

    print_part1_answer(count);
}

fn part2(input: Vec<Entry>) {
    let answer = input.iter().map(|e| get_output(e)).sum::<u32>();
    print_part2_answer(answer);
}

fn get_output(entry: &Entry) -> u32 {
    let mut digits = 0;
    let map = get_signal_mapping(&entry.unique_patterns);
    for o in &entry.output {
        digits = (digits * 10) + map[o];
    }
    digits
}

fn get_signal_mapping(patterns: &[String]) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    let one = patterns.iter().find(|e| e.len() == 2).unwrap();
    let four = patterns.iter().find(|e| e.len() == 4).unwrap();
    let seven = patterns.iter().find(|e| e.len() == 3).unwrap();
    let eight = patterns.iter().find(|e| e.len() == 7).unwrap();

    map.insert(one.to_owned(), 1);
    map.insert(four.to_owned(), 4);
    map.insert(seven.to_owned(), 7);
    map.insert(eight.to_owned(), 8);

    let mut six_chars = patterns
        .iter()
        .filter(|e| e.len() == 6)
        .cloned()
        .collect::<HashSet<_>>();

    let mut five_chars = patterns
        .iter()
        .filter(|e| e.len() == 5)
        .cloned()
        .collect::<HashSet<_>>();

    // Find 9
    let seven_and_four = seven.chars().chain(four.chars()).collect::<HashSet<_>>();
    let nine = six_chars
        .iter()
        .find(|e| char_diff(e, &seven_and_four) == 1)
        .unwrap()
        .to_owned();
    map.insert(nine.to_owned(), 9);
    six_chars.remove(&nine);

    // Find 0
    let seven = seven.chars().collect::<HashSet<_>>();
    let zero = six_chars
        .iter()
        .find(|e| char_diff(e, &seven) == 3)
        .unwrap()
        .to_owned();
    map.insert(zero.to_owned(), 0);
    six_chars.remove(&zero);

    // 6 is the last remaining one with six chars.
    map.insert(six_chars.iter().next().unwrap().to_owned(), 6);

    // Find 3
    let three = five_chars
        .iter()
        .find(|e| char_diff(e, &seven) == 2)
        .unwrap()
        .to_owned();
    map.insert(three.to_owned(), 3);
    five_chars.remove(&three);

    // Find 5
    let four = four.chars().collect::<HashSet<_>>();
    let five = five_chars
        .iter()
        .find(|e| char_diff(e, &four) == 2)
        .unwrap()
        .to_owned();
    map.insert(five.to_owned(), 5);
    five_chars.remove(&five);

    // Last one must be 2
    map.insert(five_chars.iter().next().unwrap().to_owned(), 2);

    map
}

fn char_diff(s: &str, b: &HashSet<char>) -> usize {
    let s = s.chars().collect::<HashSet<_>>();
    let diff = s.difference(&b);
    diff.count()
}

#[derive(Debug, Clone)]
struct Entry {
    unique_patterns: Vec<String>,
    output: Vec<String>,
}
