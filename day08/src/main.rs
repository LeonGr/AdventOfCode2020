use std::{collections::HashSet, io::BufRead};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

struct Status {
    final_acc: i32,
    final_ip: usize,
    finished: bool,
}

fn run_program(lines: &Vec<String>) -> Status {
    let mut instruction_pointer = 0;
    let mut accumulator = 0;

    let mut seen = HashSet::new();

    let mut finished = false;

    while !finished {
        if seen.contains(&instruction_pointer) {
            break;
        } else {
            seen.insert(instruction_pointer);
        }

        let current_line = &lines[instruction_pointer];
        let parsed = current_line.split(" ").collect::<Vec<&str>>();

        let instruction = parsed[0];

        match instruction {
            "nop" => (),
            "acc" => {
                let value = parsed[1].parse::<i32>().unwrap();
                accumulator += value;
            }
            "jmp" => {
                let value = parsed[1].parse::<i32>().unwrap();
                instruction_pointer = instruction_pointer.wrapping_add(value as usize);
                continue;
            }
            _ => panic!("Unknown instruction"),
        }

        instruction_pointer += 1;

        if instruction_pointer >= lines.len() {
            finished = true;
        }
    }

    Status {
        final_acc: accumulator,
        final_ip: instruction_pointer,
        finished: true,
    }
}

fn part1(lines: &Vec<String>) {
    let output = run_program(lines);

    println!("ip: {}", output.final_ip);
    println!("acc: {}", output.final_acc);
}

fn part2(lines: &Vec<String>) {
    let mut copy = lines.clone();
    let length = copy.len();

    for i in 0..length {
        let current_line = &copy[i];
        let parsed = current_line.split(" ").collect::<Vec<&str>>();

        let temp: String;

        match parsed[0] {
            "nop" => {
                temp = copy[i].clone();
                copy[i] = format!("{} {}", "jmp", parsed[1]);
            }
            "jmp" => {
                temp = copy[i].clone();
                copy[i] = format!("{} {}", "nop", parsed[1]);
            }
            _ => continue,
        }

        let output = run_program(&copy);

        if output.finished {
            println!("finished, acc: {}", output.final_acc);
            break;
        }

        copy[i] = temp;
    }
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
