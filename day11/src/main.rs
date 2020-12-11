use std::{collections::HashSet, io::BufRead};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{:?}", row);
    }
}

fn get_occupied_adjactent(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {

    return 0;
}

fn part1(lines: &Vec<String>) {
    let mut grid: Vec<Vec<char>> = vec![];

    grid.push(vec!['.'; lines[0].len()+2]);

    for line in lines {
        let mut padded_line: String = ".".to_owned();
        padded_line.push_str(line);
        padded_line.push_str(".");
        grid.push({
            padded_line
                .chars()
                .collect::<Vec<char>>()
        })
    }

    grid.push(vec!['.'; lines[0].len()+2]);

    print_grid(&grid);

    for i in 1..grid.len()-2 {
        
    }
}

fn part2(lines: &Vec<String>) {
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
