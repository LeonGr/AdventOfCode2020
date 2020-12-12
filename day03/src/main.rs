use std::io::BufRead;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader.lines().filter_map(std::io::Result::ok).collect())
}

fn part1(lines: &Vec<String>) {
    let width = lines[0].len();
    let height = lines.len();

    let mut y = 0;
    let mut trees = 0;

    for x in 0..height {
        let line = lines[x].chars();

        match line.clone().nth(y) {
            Some(c) if c == '#' => trees += 1,
            _ => ()
        }

        y = (y + 3) % (width);
    }

    println!("trees: {}", trees);
}

fn part2(lines: &Vec<String>) {
    let width = lines[0].len();
    let height = lines.len();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut tree_amounts: Vec<i32> = vec![];

    for slope in slopes {
        let (right, down) = slope;

        let mut y = 0;
        let mut trees = 0;

        for x in (0..height).step_by(down) {
            let line = lines[x].chars();

            match line.clone().nth(y) {
                Some(c) if c == '#' => trees += 1,
                _ => ()
            }

            y = (y + right) % (width);
        }

        tree_amounts.push(trees);
    }

    let multiplied: i32 = tree_amounts.iter().product();

    println!("Multiplied: {}", multiplied);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
