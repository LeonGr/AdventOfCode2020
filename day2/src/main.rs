use std::io::BufRead;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
       .lines()
       .filter_map(std::io::Result::ok)
       .collect())
}

fn part1(lines: &Vec<String>) {
    let mut total = 0;

    for line in lines {
        let args: Vec<&str> = line.split(" ").collect();

        let range = args[0]
            .split("-")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|string| string.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let letter = &args[1][0..1];
        let password = args[2];

        let occurrences = password.matches(letter).count() as i32;

        let valid = (range[0]..range[1]+1).contains(&occurrences);
        //let valid = occurrences >= range[0] && occurrences <= range[1];

        if valid {
            total += 1;
        }
    }

    println!("part1: {}", total);
}

fn part2(lines: &Vec<String>) {
    let mut total = 0;

    for line in lines {
        let args: Vec<&str> = line.split(" ").collect();

        let positions = args[0]
            .split("-")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|string| string.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let letter = &args[1][0..1];
        let password: Vec<&str> = args[2].split("").collect();

        let first = password[positions[0]] == letter;
        let second = password[positions[1]] == letter;

        let valid = first ^ second;

        if valid {
            total += 1;
        }
    }

    println!("part2: {}", total);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
