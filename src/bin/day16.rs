use aoc_2021::*;

const TYPE_LTERAL: u32 = 4;

fn main() {
    let input = read_input_map(|e| e.chars().collect::<Vec<char>>());
    let input = input.first().unwrap();
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: Vec<char>) {
    let bits = convert_to_bits(input);
    let mut index = 0;
    let packet = parse_packet(&bits, &mut index);
    println!("part 1: {}", sum_versions(&packet));
}

fn part2(input: Vec<char>) {
    let bits = convert_to_bits(input);
    let mut index = 0;
    let packet = parse_packet(&bits, &mut index);
    println!("part 2: {}", evaluate(&packet));
}

fn sum_versions(packet: &Packet) -> u32 {
    let mut sum = packet.header.version;

    match packet.data {
        DataType::Operator(ref packets) => {
            for p in packets.iter() {
                sum += sum_versions(p);
            }
        }
        _ => {}
    }

    sum
}

fn evaluate(packet: &Packet) -> u64 {
    let result;

    let type_id = packet.header.type_id;
    match packet.data {
        DataType::Literal(n) => {
            assert!(type_id == 4);
            result = n;
        }
        DataType::Operator(ref packets) => {
            match type_id {
                0 => {
                    // sum
                    result = packets.iter().map(|p| evaluate(p)).sum();
                }
                1 => {
                    // product
                    result = packets.iter().map(|p| evaluate(p)).product();
                }
                2 => {
                    // min
                    result = packets.iter().map(|p| evaluate(p)).min().unwrap();
                }
                3 => {
                    // max
                    result = packets.iter().map(|p| evaluate(p)).max().unwrap();
                }
                4 => {
                    panic!("invalid data type: {:?}", packet.data);
                }
                5 => {
                    // greater than
                    assert!(packets.len() == 2);
                    let v1 = evaluate(&packets[0]);
                    let v2 = evaluate(&packets[1]);
                    if v1 > v2 {
                        result = 1;
                    } else {
                        result = 0;
                    }
                }
                6 => {
                    // less than
                    assert!(packets.len() == 2);
                    let v1 = evaluate(&packets[0]);
                    let v2 = evaluate(&packets[1]);
                    if v1 < v2 {
                        result = 1;
                    } else {
                        result = 0;
                    }
                }
                7 => {
                    // equal to
                    assert!(packets.len() == 2);
                    let v1 = evaluate(&packets[0]);
                    let v2 = evaluate(&packets[1]);
                    if v1 == v2 {
                        result = 1;
                    } else {
                        result = 0;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    result
}

#[derive(Debug)]
struct Packet {
    header: Header,
    data: DataType,
}

#[derive(Debug)]
struct Header {
    version: u32,
    type_id: u32,
}

#[derive(Debug)]
enum DataType {
    Literal(u64),
    Operator(Box<[Packet]>),
}

fn parse_packet(bits: &[u8], index: &mut usize) -> Packet {
    let header = decode_header(&bits, index);

    let data = if header.type_id == TYPE_LTERAL {
        DataType::Literal(decode_literal(bits, index) as u64)
    } else {
        DataType::Operator(decode_operator(bits, index))
    };

    Packet { header, data }
}

fn decode_header(bits: &[u8], index: &mut usize) -> Header {
    let version = decode(&bits, index, 3) as u32;
    let type_id = decode(&bits, index, 3) as u32;
    Header { version, type_id }
}

fn decode_literal(bits: &[u8], index: &mut usize) -> u64 {
    let mut result = 0;
    loop {
        let prefix = bits[*index];
        *index += 1;

        result <<= 4;
        result |= decode(bits, index, 4);

        if prefix == 0 {
            break;
        }
    }

    result
}

fn decode_operator(bits: &[u8], index: &mut usize) -> Box<[Packet]> {
    let mut v = vec![];

    let length_type_id = bits[*index];
    *index += 1;
    if length_type_id == 0 {
        let length = decode(bits, index, 15) as usize;
        let end = *index + length;

        while *index < end - 1 {
            let mut sub_index = 0;
            v.push(parse_packet(&bits[*index..end], &mut sub_index));
            *index += sub_index;
        }
    } else {
        assert_eq!(length_type_id, 1);
        let count = decode(bits, index, 11) as usize;
        for _ in 0..count {
            v.push(parse_packet(&bits, index));
        }
    }

    v.into_boxed_slice()
}

fn decode(bits: &[u8], index: &mut usize, len: usize) -> u64 {
    let result = convert_to_decimal(&bits[*index..*index + len]);
    *index += len;
    result
}

fn convert_to_decimal(bits: &[u8]) -> u64 {
    assert!(bits.len() <= 64);
    let mut result = 0u64;
    for bit in bits {
        result <<= 1;
        result |= *bit as u64;
    }
    result
}

fn convert_to_bits(hex: Vec<char>) -> Vec<u8> {
    hex.iter()
        .map(|e| match e {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => unreachable!(),
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_packet(s: &str) -> Packet {
        let chars = s.chars().collect();
        let bits = convert_to_bits(chars);
        let mut index = 0;
        parse_packet(&bits, &mut index)
    }

    #[test]
    fn evaluate_1() {
        let result = evaluate(&to_packet("C200B40A82"));
        assert_eq!(result, 3);
    }

    #[test]
    fn evaluate_2() {
        let result = evaluate(&to_packet("04005AC33890"));
        assert_eq!(result, 54);
    }

    #[test]
    fn evaluate_3() {
        let result = evaluate(&to_packet("880086C3E88112"));
        assert_eq!(result, 7);
    }

    #[test]
    fn evaluate_4() {
        let result = evaluate(&to_packet("CE00C43D881120"));
        assert_eq!(result, 9);
    }

    #[test]
    fn evaluate_5() {
        let result = evaluate(&to_packet("D8005AC2A8F0"));
        assert_eq!(result, 1);
    }

    #[test]
    fn evaluate_6() {
        let result = evaluate(&to_packet("F600BC2D8F"));
        assert_eq!(result, 0);
    }

    #[test]
    fn evaluate_7() {
        let result = evaluate(&to_packet("9C005AC2F8F0"));
        assert_eq!(result, 0);
    }

    #[test]
    fn evaluate_8() {
        let result = evaluate(&to_packet("9C0141080250320F1802104A08"));
        assert_eq!(result, 1);
    }
}
