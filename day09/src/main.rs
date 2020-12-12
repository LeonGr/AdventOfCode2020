use std::io::BufRead;

fn read_input_lines_to_int() -> std::io::Result<Vec<u64>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|string| string.parse::<u64>().unwrap())
        .collect())
}

fn two_sum(input: &Vec<u64>, target: &u64) -> Option<(u64, u64)> {
    let mut copy = input.clone();

    for i in (0..copy.len()).rev() {
        let val_i = &copy[i];
        if val_i < target {
            for j in 0..copy.len() {
                let val_j = &copy[j];
                if i != j && val_i + val_j == *target {
                    return Some((copy[i], copy[j]));
                }
            }
        }
        copy.pop();
    }

    None
}

fn part1(lines: &Vec<u64>) {
    let preamble_length: usize = 25;

    let mut preamble = lines[..preamble_length].to_vec();
    let mut rest = &lines[preamble_length..];

    for number in rest {
        match two_sum(&preamble, number) {
            Some(_) => (),
            None => {
                println!("Not valid {}", number);
                break;
            }
        }

        preamble = preamble[1..].to_vec();
        preamble.push(*number);

        rest = &rest[1..];
    }
}

fn contiguous_subset_sum(input: &Vec<u64>, target: &u64) -> Option<Vec<u64>> {
    for i in 0..input.len() {
        let mut total = input[i];
        for j in i + 1..input.len() {
            total += input[j];
            if total == *target {
                let range = input[i..=j].to_vec();
                return Some(range);
            } else if total > *target {
                break;
            }
        }
    }

    None
}

fn part2(lines: &Vec<u64>) {
    let target = 2089807806;

    match contiguous_subset_sum(lines, &target) {
        Some(range) => {
            let min = range.iter().min().unwrap();
            let max = range.iter().max().unwrap();
            println!("sum smallest largest {}", min + max);
        }
        None => unreachable!(),
    }
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines_to_int()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
