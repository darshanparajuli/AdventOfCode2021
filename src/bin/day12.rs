use aoc_2021::*;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = parse_input();
    part1(input.clone());
    part2(input);
}

fn part1(input: HashMap<Cave, Vec<Cave>>) {
    let mut count = 0;
    let mut queue: VecDeque<(Cave, HashSet<Cave>)> = VecDeque::new();

    queue.push_back((Cave::Start, HashSet::new()));

    while let Some((cave, mut visited)) = queue.pop_front() {
        if cave == Cave::End {
            count += 1;
            continue;
        }

        if visited.contains(&cave) {
            continue;
        }

        if cave.is_small() {
            visited.insert(cave.clone());
        }

        if let Some(connections) = input.get(&cave) {
            for c in connections {
                queue.push_back((c.clone(), visited.clone()));
            }
        }
    }

    print_part1_answer(count);
}

fn part2(input: HashMap<Cave, Vec<Cave>>) {
    let mut queue: VecDeque<(Cave, HashMap<Cave, u32>, bool)> = VecDeque::new();
    let mut count = 0;

    queue.push_back((Cave::Start, HashMap::new(), false));

    while let Some((cave, mut visited, mut visited_twice)) = queue.pop_front() {
        if cave == Cave::End {
            count += 1;
            continue;
        }

        if visited_twice {
            if let Some(count) = visited.get(&cave) {
                if *count > 0 {
                    continue;
                }
            }
        } else {
            if let Some(count) = visited.get(&cave) {
                if *count > 1 {
                    continue;
                }
            }
        }

        if cave.is_small() {
            let count = visited.entry(cave.clone()).or_insert(0);
            *count += 1;
            if *count > 1 {
                visited_twice = true;
            }
        }

        if let Some(connections) = input.get(&cave) {
            for c in connections {
                queue.push_back((c.clone(), visited.clone(), visited_twice));
            }
        }
    }

    print_part2_answer(count);
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Cave {
    Start,
    Small(String),
    Big(String),
    End,
}

impl Cave {
    fn is_small(&self) -> bool {
        match *self {
            Cave::Small(_) => true,
            _ => false,
        }
    }

    fn is_start(&self) -> bool {
        match *self {
            Cave::Start => true,
            _ => false,
        }
    }

    fn is_end(&self) -> bool {
        match *self {
            Cave::End => true,
            _ => false,
        }
    }
}

fn parse_input() -> HashMap<Cave, Vec<Cave>> {
    let mut map = HashMap::new();

    let to_cave = |c| match c {
        "start" => Cave::Start,
        "end" => Cave::End,
        _ => {
            let ch = c.chars().next().unwrap();
            if ch.is_uppercase() {
                Cave::Big(c.to_owned())
            } else {
                Cave::Small(c.to_owned())
            }
        }
    };

    let input = read_input_map(|s| {
        let mut it = s.split("-");
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        if b == "start" || a == "end" {
            (b.to_owned(), a.to_owned())
        } else {
            (a.to_owned(), b.to_owned())
        }
    });

    for (a, b) in &input {
        map.entry(to_cave(&a))
            .or_insert_with(|| vec![])
            .push(to_cave(&b));

        if !to_cave(b).is_end() && !to_cave(a).is_start() {
            map.entry(to_cave(&b))
                .or_insert_with(|| vec![])
                .push(to_cave(&a));
        }
    }

    // Nothing points to End
    assert_eq!(map.get(&Cave::End), None);

    // Nothing points back to Start
    map.keys().for_each(|k| {
        assert!(map
            .get(k)
            .unwrap()
            .iter()
            .find(|e| **e == Cave::Start)
            .is_none());
    });

    map
}
