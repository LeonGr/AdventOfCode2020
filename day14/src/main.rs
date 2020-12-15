use std::{io::BufRead, collections::HashMap};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn apply_mask(mask: &str, value: u64) -> u64 {
    let chars = mask.chars().collect::<Vec<char>>();
    let mask_len = chars.len();
    let mut ones_mask: u64 = 0;
    let mut zeroes_mask: u64 = 0;

    for i in 0..mask_len {
        match chars[mask_len - i - 1] {
            'X' => continue,
            '1' => ones_mask += (2u64).pow(i as u32),
            '0' => zeroes_mask += (2u64).pow(i as u32),
            _ => unreachable!(),
        }
    }

    (value | ones_mask) & !zeroes_mask
}

fn part1(lines: &Vec<String>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut current_mask: &str = "";
    for line in lines {
        if line.starts_with("mask") {
            current_mask = line.split(" = ").collect::<Vec<&str>>().pop().unwrap();
        } else {
            let values = line.get(4..).unwrap().split("] = ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            memory.insert(values[0], apply_mask(current_mask, values[1]));
        }
    }

    println!("{:?}", memory.values().into_iter().sum::<u64>());
}

fn apply_mask2(mask: &str, address: u64) -> String {
    let binary = format!("{:b}", address);
    let zeroes = vec!['0'; 36 - binary.len()].iter().collect::<String>();
    let with_leading_zeroes = format!("{}{}", zeroes, binary).chars().collect::<Vec<char>>();
    let mask_chars = mask.chars().collect::<Vec<char>>();

    let mut output: Vec<char> = vec![];

    for i in 0..36 {
        if mask_chars[i] == '0' {
            output.push(with_leading_zeroes[i]);
        } else {
            output.push(mask_chars[i]);
        }
    }

    output.iter().collect::<String>()
}

fn with_x_to_all(floating: &str) -> Vec<String> {
    let mut current: Vec<String> = vec!["".to_string()];

    for bit in floating.chars().rev() {
        if bit == 'X' {
            let mut copy = current.clone();
            for i in 0..current.len() {
                current[i] = format!("{}1", current[i]);
                copy[i] = format!("{}0", copy[i]);
            }
            current.append(&mut copy);
        }
        else {
            for i in 0..current.len() {
                current[i] = format!("{}{}", current[i], bit);
            }
        }
    }

    for i in 0..current.len() {
        current[i] = current[i].chars().rev().collect::<String>();
    }

    current
}

fn part2(lines: &Vec<String>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut current_mask: &str = "";

    for line in lines {
        if line.starts_with("mask") {
            current_mask = line.split(" = ").collect::<Vec<&str>>().pop().unwrap();
        } else {
            let values = line.get(4..).unwrap().split("] = ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let with_x = apply_mask2(current_mask, values[0]);
            let all_addresses = with_x_to_all(with_x.as_str());

            for address in all_addresses {
                let address_decimal = u64::from_str_radix(address.as_str(), 2).unwrap();
                memory.insert(address_decimal, values[1]);
            }

        }
    }

    println!("{:?}", memory.values().into_iter().sum::<u64>());
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
