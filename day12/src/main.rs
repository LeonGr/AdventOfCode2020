use std::io::BufRead;

use ndarray::{Array1, arr2};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

enum Part {
    P1,
    P2,
}

fn navigate(lines: &Vec<String>, part: Part) -> i32 {
    let mut position = Array1::from(vec![0, 0]);
    let mut direction = match part {
        Part::P1 => Array1::from(vec![1, 0]),
        Part::P2 => Array1::from(vec![10, 1]),
    };

    for line in lines {
        let mut chars = line.chars();
        let action = chars.nth(0).unwrap();
        let mut value = chars.skip(0).collect::<String>().parse::<i32>().unwrap();

        let mut delta: Option<Array1<i32>> = None;

        match action {
            'F' => position = position + value * &direction,
            'N' => {
                delta = Some(Array1::from(vec![0, value]));
            },
            'E' => {
                delta = Some(Array1::from(vec![value, 0]));
            },
            'S' => {
                delta = Some(Array1::from(vec![0, -value]));
            },
            'W' => {
                delta = Some(Array1::from(vec![-value, 0]));
            },
            'L' | 'R' => {
                if action == 'R' {
                    value *= -1;
                }
                let angle = (value as f64).to_radians();
                let cos_a = angle.cos() as i32;
                let sin_a = angle.sin() as i32;
                let rotation_matrix = arr2(&[[cos_a, -sin_a], [sin_a, cos_a]]);

                direction = rotation_matrix.dot(&direction);
            }
            _ => unreachable!(),
        }

        match delta {
            Some(delta) => {
                match part {
                    Part::P1 => position = position + delta,
                    Part::P2 => direction = direction + delta,
                }
            }
            None => continue,
        }
    }

    let x = position.get(0).unwrap();
    let y = position.get(1).unwrap();

    x.abs() + y.abs()
}

fn part1(lines: &Vec<String>) {
    println!("{}", navigate(lines, Part::P1));
}

fn part2(lines: &Vec<String>) {
    println!("{}", navigate(lines, Part::P2));
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
