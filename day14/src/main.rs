use std::io::BufRead;

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
}

fn part2(lines: &Vec<String>) {
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    println!("{}", apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 11));
    println!("{}", apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 101));
    println!("{}", apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 0));

    Ok(())
}
