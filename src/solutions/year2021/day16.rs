// Advent of Code 2021 - Day 16

use std::collections::HashMap;

fn bin_to_usize(bin: &str) -> usize {
    let val: usize = bin.chars().rev().enumerate().fold(0, |acc, (idx, bit)| {
        let bit_num = bit.to_digit(10).unwrap();
        acc + (bit_num as usize * usize::pow(2, idx as u32))
    });
    val
}

fn parse(input: &String) -> String {
    let parse_dict: HashMap<char, &str> = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);
    return input
        .chars()
        .map(|x| parse_dict.get(&x).unwrap().to_string())
        .collect();
}

enum Instructions {
    VERSION,
    TypeId,
    LITERAL,
    OPERATOR,
    END,
}

#[derive(Debug)]
struct Packet {
    version: usize,
    value: usize,
    sub_packets: Vec<Packet>,
}

fn read(data: &String, start: usize, limit: usize) -> (Packet, usize) {
    let mut cursor = start;
    let mut next = Instructions::VERSION;
    let (mut version, mut type_id, mut literal_value_parts) = (0, 0, vec![]);
    let mut sub_packets: Vec<Packet> = vec![];
    while cursor < limit {
        match next {
            Instructions::VERSION => {
                let val = &data[cursor..cursor + 3];
                version = bin_to_usize(val);
                next = Instructions::TypeId;
                cursor += 3;
            }
            Instructions::TypeId => {
                let val = &data[cursor..cursor + 3];
                type_id = bin_to_usize(val);
                next = match type_id {
                    4 => Instructions::LITERAL,
                    _ => Instructions::OPERATOR,
                };
                cursor += 3;
            }
            Instructions::LITERAL => {
                let val = &data[cursor..cursor + 5];
                literal_value_parts.push(val[1..].to_string());
                if val.chars().nth(0).unwrap() == '0' {
                    cursor += 5;
                    break;
                } else {
                    cursor += 5;
                }
            }
            Instructions::OPERATOR => {
                let val = &data[cursor..cursor + 1];
                cursor += 1;
                let (bit_type, bit_length) = match val {
                    "0" => ("total_length", 15),
                    "1" => ("count", 11),
                    _ => break,
                };
                let sub_packet_size = bin_to_usize(&data[cursor..cursor + bit_length]);
                cursor += bit_length;
                if bit_type == "total_length" {
                    sub_packets.extend(sub_packets_by_length(
                        data,
                        cursor,
                        cursor + sub_packet_size,
                    ));
                    cursor += sub_packet_size;
                } else {
                    let (packets, counter) =
                        sub_packets_by_count(data, cursor, limit, sub_packet_size);
                    sub_packets.extend(packets);
                    cursor = counter
                }
                next = Instructions::END
            }
            Instructions::END => break,
        }
    }

    let sub_packet_values = sub_packets.iter().map(|x| x.value);
    let value = match type_id {
        0 => sub_packet_values.sum(),
        1 => sub_packet_values.product(),
        2 => sub_packet_values.min().unwrap(),
        3 => sub_packet_values.max().unwrap(),
        4 => bin_to_usize(&literal_value_parts.join("")),
        5 => {
            let v_vec: Vec<usize> = sub_packet_values.collect();
            if v_vec[0] > v_vec[1] {
                1
            } else {
                0
            }
        }
        6 => {
            let v_vec: Vec<usize> = sub_packet_values.collect();
            if v_vec[0] < v_vec[1] {
                1
            } else {
                0
            }
        }
        7 => {
            let v_vec: Vec<usize> = sub_packet_values.collect();
            if v_vec[0] == v_vec[1] {
                1
            } else {
                0
            }
        }
        _ => 0,
    };
    (
        Packet {
            version,
            value,
            sub_packets,
        },
        cursor,
    )
}

fn sub_packets_by_count(
    data: &String,
    start: usize,
    limit: usize,
    total: usize,
) -> (Vec<Packet>, usize) {
    let mut cursor = start;
    let mut sub_packets: Vec<Packet> = vec![];
    while sub_packets.len() < total {
        let (pkt, end) = read(data, cursor, limit);
        cursor = end;
        sub_packets.push(pkt)
    }
    (sub_packets, cursor)
}

fn sub_packets_by_length(data: &String, start: usize, limit: usize) -> Vec<Packet> {
    let mut cursor = start;
    let mut sub_packets: Vec<Packet> = vec![];
    while cursor < limit {
        let (pkt, end) = read(data, cursor, limit);
        cursor = end;
        sub_packets.push(pkt)
    }
    sub_packets
}

fn version_sum(p: &Packet) -> usize {
    let mut sum = p.version;
    for sub_packet in &p.sub_packets {
        sum += version_sum(&sub_packet);
    }
    sum
}

fn solve(input: &String) -> (usize, usize) {
    let parsed_data = parse(input);
    let (result, _) = read(&parsed_data, 0, parsed_data.len());
    let sum = version_sum(&result);
    (result.value, sum)
}

pub fn solution_2021_16_01(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(filepath)?;
    let (_, version_sum) = solve(&input.trim().to_string());
    Ok(version_sum as i64)
}

pub fn solution_2021_16_02(filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(filepath)?;
    let (value, _) = solve(&input.trim().to_string());
    Ok(value as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2021_16_01() {
        let result = solution_2021_16_01("inputs/2021/day16e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_16_01() {
        let result = solution_2021_16_01("inputs/2021/day16.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn test_2021_16_02() {
        let result = solution_2021_16_02("inputs/2021/day16e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_16_02() {
        let result = solution_2021_16_02("inputs/2021/day16.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
