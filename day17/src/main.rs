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
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input {
        grid.push(line.chars().collect())
    }

    grid
}

fn add_padding(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut padded_grid: Vec<Vec<char>> = vec![];

    padded_grid.push(vec!['.'; grid[0].len() + 2]);
    for line in grid {
        padded_grid.push(format!(".{}.", line.iter().collect::<String>()).chars().collect())
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
    universe
        .iter()
        .map(|grid| count_occupied(grid))
        .sum()
}

fn get_occupied_adjactent(universe: &Vec<Vec<Vec<char>>>, x: i8, y: i8, z: i8) -> usize {
    iproduct!(-1..=1, -1..=1, -1..=1)
        .filter(|t| t != &(0, 0, 0))
        .filter(|(dx, dy, dz)| {
            let position: (usize, usize, usize) = ((x + dx) as usize, (y + dy) as usize, (z + dz) as usize);
            //println!("{:?}", position);
            universe[position.2][position.0][position.1] == '#'
        })
        .count()
}

fn part1(lines: &Vec<String>) {
    let grid = add_padding(&add_padding(&to_char(&lines)));
    let empty_layer = vec![vec!['.'; grid[0].len()]; grid.len()];
    print_grid(&grid);
    println!("");
    print_grid(&empty_layer);

    let mut universe = vec![empty_layer.clone(), empty_layer.clone(), grid.clone(), empty_layer.clone(), empty_layer.clone()];
    //println!("{}", get_occupied_adjactent(&universe, 2, 2, 1));

    let mut width: usize;
    let mut height: usize;
    let mut depth: usize;

    for i in 0..6 {
        println!("cycle {}", i+1);

        width = universe[0].len();
        height = universe[0][0].len();
        depth = universe.len();

        let empty_layer = &universe[0].clone();
        let empty_padded_layer = add_padding(&universe[0].clone());
        let mut new_universe = vec![empty_padded_layer.clone(), empty_padded_layer.clone()];

        for z in 1..(depth - 1) {
            let mut new_layer = empty_layer.clone();

            for x in 1..(width - 1) {
                for y in 1..(height - 1) {
                    //println!("creating {:?}", (x, y, z));
                    let active_neighbours = get_occupied_adjactent(&universe, x as i8, y as i8, z as i8);
                    if universe[z][x][y] == '#' {
                        if active_neighbours == 2 || active_neighbours == 3 {
                            new_layer[x][y] = '#';
                        } else {
                            new_layer[x][y] = '.';
                        }
                    } else if universe[z][x][y] == '.' && active_neighbours == 3 {
                        new_layer[x][y] = '#';
                    }
                    //print_grid(&new_layer);
                }
            }

            new_universe.push(add_padding(&new_layer));
        }

        new_universe.append(&mut vec![empty_padded_layer.clone(), empty_padded_layer.clone()]);

        for layer in &new_universe {
            println!("");
            print_grid(layer);
        }

        universe = new_universe;
    }

    println!("count_occupied_3d {}", count_occupied_3d(&universe));
}

fn part2(lines: &Vec<String>) {

}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;


    part1(&lines);
    part2(&lines);

    Ok(())
}
