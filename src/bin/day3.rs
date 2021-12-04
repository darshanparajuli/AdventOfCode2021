use aoc_2021::*;

fn main() {
    let input = read_input_map(|s| {
        s.chars()
            .map(|e| e.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    });

    part1(&input);
    part2(&input);
}

fn part1(input: &[Vec<u32>]) {
    let mut gamma = 0;
    let mut epsilon = 0;

    for k in 0..input.first().unwrap().len() {
        let ones = input.iter().map(|e| e[k]).sum::<u32>();
        let zeroes = input.len() as u32 - ones;

        if k > 0 {
            gamma <<= 1;
            epsilon <<= 1;
        }

        if ones > zeroes {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }

    print_part1_answer(gamma * epsilon);
}

fn part2(input: &[Vec<u32>]) {
    let o2_rating = get_rating(input.to_vec(), |ones, zeros| {
        if ones > zeros {
            1
        } else if zeros > ones {
            0
        } else {
            1
        }
    });

    let co2_rating = get_rating(input.to_vec(), |ones, zeros| {
        if ones > zeros {
            0
        } else if zeros > ones {
            1
        } else {
            0
        }
    });

    print_part2_answer(o2_rating * co2_rating);
}

fn get_rating(mut bits: Vec<Vec<u32>>, cmp: fn(u32, u32) -> u32) -> u32 {
    for k in 0..bits.first().unwrap().len() {
        let ones = bits.iter().map(|e| e[k]).sum::<u32>();
        let zeroes = bits.len() as u32 - ones;
        let common = cmp(ones, zeroes);

        let filtered = bits
            .iter()
            .filter(|e| e[k] == common)
            .cloned()
            .collect::<Vec<_>>();
        bits = filtered;

        if bits.len() == 1 {
            break;
        }
    }
    assert_eq!(bits.len(), 1);

    bits_to_u32(bits.first().unwrap())
}

fn bits_to_u32(bits: &[u32]) -> u32 {
    let str_value = bits.iter().map(|e| e.to_string()).collect::<String>();
    u32::from_str_radix(&str_value, 2).unwrap()
}
