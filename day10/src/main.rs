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

fn tribonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    } else {
        return tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3);
    }
}

fn part2(lines: &Vec<u64>) {
    let mut total: u64 = 1;

    let mut tribonaccis: Vec<u64> = vec![];

    for i in 2..7 {
        tribonaccis.push(tribonacci(i as u64));
    }

    let mut consecutive_one_diffs = 0;

    for i in 1..lines.len() {
        let diff = lines[i] - lines[i - 1];

        if diff == 1 {
            consecutive_one_diffs += 1;
        } else {
            let options;

            if diff == 3 && consecutive_one_diffs > 0 {
                options = tribonaccis[consecutive_one_diffs];
            } else if consecutive_one_diffs > 0 {
                options = tribonaccis[consecutive_one_diffs] + tribonaccis[consecutive_one_diffs-1];
            } else {
                options = 1;
            }

            total *= options;
            consecutive_one_diffs = 0;
        }
    }

    println!("final {}", total * tribonaccis[consecutive_one_diffs]);
}

fn main() -> std::io::Result<()> {
    let mut lines = read_input_lines_to_int()?;

    lines.push(0);
    lines.push(lines.iter().max().unwrap() + 3);
    lines.sort();

    part1(&lines);
    part2(&lines);

    Ok(())
}
