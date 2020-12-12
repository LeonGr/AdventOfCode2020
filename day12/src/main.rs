use std::{collections::HashSet, io::BufRead};

use ndarray::{Array, arr2};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn part1(lines: &Vec<String>) {
    let mut position = Array::from(vec![0, 0]);
    let mut direction = Array::from(vec![1, 0]);

    for line in lines {
        let mut chars = line.chars();
        let action = chars.nth(0).unwrap();
        let mut value = chars.skip(0).collect::<String>().parse::<i32>().unwrap();

        match action {
            'F' => position = position + value * &direction,
            'N' => position = position + Array::from(vec![0, value]),
            'E' => position = position + Array::from(vec![value, 0]),
            'S' => position = position + Array::from(vec![0, -value]),
            'W' => position = position + Array::from(vec![-value, 0]),
            'L' | 'R' => {
                if action == 'R' {
                    value *= -1;
                }
                let angle = (value as f64).to_radians();
                let ca = angle.cos() as i32;
                let sa = angle.sin() as i32;
                let rotation_matrix = arr2(&[[ca, -sa], [sa, ca]]);
                direction = rotation_matrix.dot(&direction);
            }
            _ => unreachable!(),
        }
    }

    let x = position.get(0).unwrap();
    let y = position.get(1).unwrap();
    println!("{}", x.abs() + y.abs());
}

fn part2(lines: &Vec<String>) {
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
