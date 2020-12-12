use std::io::BufRead;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader.lines().filter_map(std::io::Result::ok).collect())
}

fn binary(numbers: Vec<char>, first: char, second: char, max: u16) -> u16 {
    numbers
        .iter()
        .fold(0..max, |mut acc, &row| {
            let half = (acc.end - acc.start) / 2;
            if row == first {
                acc.end -= half;
            } else if row == second {
                acc.start += half;
            }
            acc
        }).start
}

fn get_seat_id(pass: &String) -> u16 {
    let rows: Vec<char> = pass.chars().take(7).collect();
    let cols: Vec<char> = pass.chars().skip(7).take(3).collect();

    let row_number = binary(rows, 'F', 'B', 128);
    let col_number = binary(cols, 'L', 'R', 8);

    row_number * 8 + col_number
}

fn part1(lines: &Vec<String>) {
    let max_seat_id = lines
        .iter()
        .map(|line| get_seat_id(line))
        .max()
        .unwrap();

    println!("{}", max_seat_id);
}

fn part2(lines: &Vec<String>) {
    let mut all_seat_ids: Vec<u16> = lines
        .into_iter()
        .map(|line| get_seat_id(line))
        .collect();

    all_seat_ids.sort();

    let mut current = all_seat_ids[0];
    for id in &all_seat_ids {
        if id != &current {
            break;
        } else {
            current += 1;
        }
    }

    println!("My seat: {}", current);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
