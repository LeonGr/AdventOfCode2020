use std::io::BufRead;

fn read_input_lines_to_int() -> std::io::Result<Vec<u64>> {
    let input_file = std::fs::File::open("input_test")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|string| string.parse::<u64>().unwrap())
        .collect())
}

fn part1(lines: &Vec<u64>) {

    let mut diff_one = 0;
    let mut diff_three = 0;

    for i in 0..lines.len()-1 {
        match lines[i + 1] - lines[i] {
            1 => diff_one += 1,
            3 => diff_three += 1,
            _ => unreachable!(),
        }
    }

    println!("one: {} three: {}", diff_one, diff_three);
    println!("multiplied: {}", diff_one * diff_three);
}

fn part2(lines: &Vec<u64>, start: usize) -> u64 {
    let mut total: u64 = 0;
    //println!("{:?}", lines);

    for i in start..lines.len()-2 {
        if lines[i] - lines[i - 1] < 3 && lines[i + 1] - lines[i - 1] <= 3 {
            let mut copy = lines.clone();
            copy.remove(i);
            total += 1;
            total += part2(&copy, i);
        }
    }

    return total;
}

fn main() -> std::io::Result<()> {
    let mut lines = read_input_lines_to_int()?;

    lines.push(0);
    lines.push(lines.iter().max().unwrap() + 3);
    lines.sort();

    //for line in &lines {
        //println!("{}", line);
    //}

    part1(&lines);
    println!("{}", part2(&lines, 1) + 1);

    Ok(())
}
