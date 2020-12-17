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

fn to_char(input: &Vec<String>) -> Vec<Vec<char>> {
    input
        .iter()
        .fold(vec![], |mut acc, line| {
            acc.push(line.chars().collect());
            acc
        })
}

fn add_padding(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut padded_grid: Vec<Vec<char>> = vec![];

    padded_grid.push(vec!['.'; grid[0].len() + 2]);
    for line in grid {
        padded_grid.push(
            format!(".{}.", line.iter().collect::<String>())
                .chars()
                .collect(),
        )
    }
    padded_grid.push(vec!['.'; grid[0].len() + 2]);

    padded_grid
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{:?}", row);
    }
}

fn count_occupied(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .map(|row| row.iter().collect::<String>().matches("#").count())
        .sum()
}

fn count_occupied_3d(universe: &Vec<Vec<Vec<char>>>) -> usize {
    universe.iter().map(|grid| count_occupied(grid)).sum()
}

fn count_occupied_4d(space_time: &Vec<Vec<Vec<Vec<char>>>>) -> usize {
    space_time
        .iter()
        .map(|universe| count_occupied_3d(universe))
        .sum()
}

fn get_occupied_adjactent_3d(universe: &Vec<Vec<Vec<char>>>, x: i8, y: i8, z: i8) -> usize {
    iproduct!(-1..=1, -1..=1, -1..=1)
        .filter(|t| t != &(0, 0, 0))
        .filter(|(dx, dy, dz)| {
            let position: (usize, usize, usize) =
                ((x + dx) as usize, (y + dy) as usize, (z + dz) as usize);
            universe[position.2][position.0][position.1] == '#'
        })
        .count()
}

fn get_occupied_adjactent_4d(space_time: &Vec<Vec<Vec<Vec<char>>>>, x: i8, y: i8, z: i8, w: i8) -> usize {
    iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
        .filter(|t| t != &(0, 0, 0, 0))
        .filter(|(dx, dy, dz, dw)| {
            let position: (usize, usize, usize, usize) =
                ((x + dx) as usize, (y + dy) as usize, (z + dz) as usize, (w + dw) as usize);
            space_time[position.3][position.2][position.0][position.1] == '#'
        })
        .count()
}

fn part1(lines: &Vec<String>) {
    let grid = add_padding(&add_padding(&to_char(&lines)));
    let empty_layer = vec![vec!['.'; grid[0].len()]; grid.len()];

    let mut universe = vec![empty_layer; 5];
    universe[2] = grid;

    for _ in 0..6 {
        let empty_padded_layer = add_padding(&universe[0]);
        let mut new_universe = vec![empty_padded_layer.clone(); 2];

        for z in 1..(universe.len() - 1) {
            let mut new_layer = empty_padded_layer.clone();

            for x in 1..(universe[0].len() - 1) {
                for y in 1..(universe[0][0].len() - 1) {
                    let active_neighbours =
                        get_occupied_adjactent_3d(&universe, x as i8, y as i8, z as i8);
                    if universe[z][x][y] == '#' {
                        if active_neighbours == 2 || active_neighbours == 3 {
                            new_layer[x][y] = '#';
                        } else {
                            new_layer[x][y] = '.';
                        }
                    } else if universe[z][x][y] == '.' && active_neighbours == 3 {
                        new_layer[x][y] = '#';
                    }
                }
            }

            new_universe.push(add_padding(&new_layer));
        }

        new_universe.append(&mut vec![empty_padded_layer; 2]);

        universe = new_universe;
    }

    println!("count_occupied_3d {}", count_occupied_3d(&universe));
}

fn part2(lines: &Vec<String>) {
    let grid = add_padding(&add_padding(&to_char(&lines)));
    let initial_empty_layer = vec![vec!['.'; grid[0].len()]; grid.len()];

    let mut universe = vec![initial_empty_layer.clone(); 5];
    universe[2] = grid;

    let empty_universe = vec![initial_empty_layer; 5];

    let mut space_time = vec![empty_universe; 5];
    space_time[2] = universe;

    for i in 0..6 {
        let empty_layer = &space_time[0][0];
        let empty_padded_layer = add_padding(&empty_layer);
        let empty_universe = vec![empty_padded_layer.clone(); 7 + (2 * i)];

        let mut new_space_time = vec![empty_universe.clone(); 2];

        for w in 1..(space_time.len() - 1) {
            let mut new_universe = vec![empty_padded_layer.clone(); 2];

            for z in 1..(space_time[0].len() - 1) {
                let mut new_layer = empty_layer.clone();

                for x in 1..(space_time[0][0].len() - 1) {
                    for y in 1..(space_time[0][0][0].len() - 1) {
                        let active_neighbours = get_occupied_adjactent_4d(&space_time, x as i8, y as i8, z as i8, w as i8);

                        if space_time[w][z][x][y] == '#' {
                            if active_neighbours == 2 || active_neighbours == 3 {
                                new_layer[x][y] = '#';
                            } else {
                                new_layer[x][y] = '.';
                            }
                        } else if space_time[w][z][x][y] == '.' && active_neighbours == 3 {
                            new_layer[x][y] = '#';
                        }
                    }
                }

                new_universe.push(add_padding(&new_layer));
            }

            new_universe.append(&mut vec![empty_padded_layer.clone(); 2]);

            new_space_time.push(new_universe);
        }

        new_space_time.append(&mut vec![empty_universe; 2]);

        space_time = new_space_time;
    }

    println!("count_occupied_4d {}", count_occupied_4d(&space_time));
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
