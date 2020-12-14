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
            //println!("{}", current_mask);
        } else {
            //println!("{:?}", line.split("mem[").split("] = ").collect::<Vec<&str>>());
            let values = line.get(4..).unwrap().split("] = ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            //println!("{:?}", values);

            memory.insert(values[0], apply_mask(current_mask, values[1]));
        }
    }

    println!("{:?}", memory.values().into_iter().sum::<u64>());
}

fn part2(lines: &Vec<String>) {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut current_masks: Vec<String> = vec!["".to_string()];
    for line in lines {
        println!("");
        if line.starts_with("mask") {
            current_masks = vec!["".to_string()];

            let mask = line.split(" = ").collect::<Vec<&str>>().pop().unwrap().chars();

            for bit in mask.rev() {
                if bit == 'X' {
                    let mut copy = current_masks.clone();
                    for i in 0..current_masks.len() {
                        current_masks[i] = format!("{}1", current_masks[i]);
                        copy[i] = format!("{}0", copy[i]);
                    }
                    current_masks.append(&mut copy);
                }
                else {
                    for i in 0..current_masks.len() {
                        current_masks[i] = format!("{}{}", current_masks[i], bit);
                    }
                }
            }

            for i in 0..current_masks.len() {
                current_masks[i] = current_masks[i].chars().rev().collect::<String>();
            }

            println!("{:?}", current_masks);
        } else {
            //println!("{:?}", line.split("mem[").split("] = ").collect::<Vec<&str>>());
            let values = line.get(4..).unwrap().split("] = ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            println!("{:?}", values);

            for mask in &current_masks {
                let decoded_address = apply_mask(mask.as_str(), values[0]);
                println!("{} -> {}", values[0], decoded_address);
                memory.insert(decoded_address, values[1]);
            }

            //memory.insert(values[0], apply_mask(current_mask, values[1]));
        }
    }

    println!("{:?}", memory.values().into_iter().sum::<u64>());
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    //println!("{}", apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 11));
    //println!("{}", apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 101));
    //println!("{}", apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 0));

    Ok(())
}
