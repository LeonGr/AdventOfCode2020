use std::io::BufRead;
#[macro_use]
extern crate itertools;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn to_char_with_padding(input: &Vec<String>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];

    grid.push(vec!['.'; input[0].len() + 2]);
    for line in input {
        grid.push(format!(".{}.", line).chars().collect())
    }
    grid.push(vec!['.'; input[0].len() + 2]);

    grid
}

fn count_occupied(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .map(|row| row.iter().collect::<String>().matches("#").count())
        .sum()
}

fn get_occupied_adjactent(grid: &Vec<Vec<char>>, row: i32, col: i32) -> usize {
    iproduct!(-1..=1, -1..=1)
        .filter(|t| t != &(0, 0))
        .filter(|(dr, dc)| grid[(row + dr) as usize][(col + dc) as usize] == '#')
        .count()
}

fn part1(lines: &Vec<String>) {
    let mut grid = to_char_with_padding(lines);
    let mut changed;

    loop {
        changed = false;

        let mut copy = grid.clone();

        for i in 1..copy.len() - 1 {
            let row = &mut copy[i];

            for j in 1..row.len() - 1 {
                if row[j] == 'L' && get_occupied_adjactent(&grid, i as i32, j as i32) == 0 {
                    row[j] = '#';
                    changed = true;
                } else if row[j] == '#' && get_occupied_adjactent(&grid, i as i32, j as i32) >= 4 {
                    row[j] = 'L';
                    changed = true;
                }
            }
        }

        grid = copy;

        if !changed {
            break;
        }
    }

    println!("part 1: occupied {}", count_occupied(&grid));
}

fn get_occupied_visible(grid: &Vec<Vec<char>>, row: i32, col: i32) -> usize {
    iproduct!(-1..=1, -1..=1)
        .filter(|t| t != &(0, 0))
        .filter(|(dr, dc)| {
            let mut distance = 1;

            loop {
                let coordinate_x = row + dr * distance;
                let coordinate_y = col + dc * distance;

                if coordinate_x < 0
                    || coordinate_y < 0
                    || coordinate_x >= grid.len() as i32
                    || coordinate_y >= grid[0].len() as i32
                {
                    return false;
                }

                match grid[coordinate_x as usize][coordinate_y as usize] {
                    '#' => return true,
                    'L' => return false,
                    _ => distance += 1,
                }
            }
        })
        .count()
}

fn part2(lines: &Vec<String>) {
    let mut grid = to_char_with_padding(lines);
    let mut changed;

    loop {
        changed = false;

        let mut copy = grid.clone();

        for i in 1..copy.len() - 1 {
            let row = &mut copy[i];

            for j in 1..row.len() - 1 {
                if row[j] == 'L' && get_occupied_visible(&grid, i as i32, j as i32) == 0 {
                    row[j] = '#';
                    changed = true;
                } else if row[j] == '#' && get_occupied_visible(&grid, i as i32, j as i32) >= 5 {
                    row[j] = 'L';
                    changed = true;
                }
            }
        }

        grid = copy;

        if !changed {
            break;
        }
    }

    println!("part 2: occupied {}", count_occupied(&grid));
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
