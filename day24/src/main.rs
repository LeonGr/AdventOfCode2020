use std::{ io::BufRead, collections::HashMap };

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

#[derive(Debug, Clone)]
enum Color {
    White,
    Black,
}

type Tile = (i8, i8, i8);

fn parse(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut output: Vec<Vec<String>> = vec![];

    for line in &mut lines.clone() {
        let mut direction: Vec<String> = vec![];

        let mut split = line.split("");

        loop {
            match split.next() {
                Some(c) if c == "" => continue,
                Some(c) if c == "e" || c == "w" => direction.push(c.to_string()),
                Some(c) if c != "" => {
                    let two_chars = format!("{}{}", c, split.next().unwrap());
                    direction.push(two_chars);
                }
                _ => break,
            }
        }
        output.push(direction);
    }

    output
}

fn move_direction(direction: &String, coordinate: Tile) -> Tile {
    let (x, y, z) = coordinate;
    match direction.as_str() {
        "e"  => (x+1 , y-1 , z   ),
        "se" => (x   , y-1 , z+1 ),
        "sw" => (x-1 , y   , z+1 ),
        "w"  => (x-1 , y+1 , z   ),
        "nw" => (x   , y+1 , z-1 ),
        "ne" => (x+1 , y   , z-1 ),
        _ => unreachable!(),
    }
}

fn part1(instructions: &Vec<Vec<String>>) -> HashMap<Tile, Color> {
    let mut tiles: HashMap<Tile, Color> = HashMap::new();
    let reference: Tile = (0, 0, 0);
    tiles.insert(reference.clone(), Color::White);

    for instruction in instructions {
        let mut current = reference;

        let mut step = instruction.iter();
        loop {
            match step.next() {
                Some(direction) => current = move_direction(direction, current),
                None => break,
            }
        }

        match tiles.get(&current) {
            Some(Color::Black) => { tiles.insert(current, Color::White); }
            _ =>                  { tiles.insert(current, Color::Black); }
        }
    }

    let mut total_black = 0;
    for color in tiles.values() {
        match color {
            Color::Black => total_black += 1,
            Color::White => (),
        }
    }

    println!("total {}", total_black);

    tiles
}

fn neighbours((x, y, z): Tile) -> Vec<Tile> {
    vec![
        (x+1 , y-1 , z   ),
        (x   , y-1 , z+1 ),
        (x-1 , y   , z+1 ),
        (x-1 , y+1 , z   ),
        (x   , y+1 , z-1 ),
        (x+1 , y   , z-1 ),
    ]
}

fn count_black_neighbours(coordinate: Tile, tiles: &HashMap<Tile, Color>) -> u8 {
    neighbours(coordinate)
        .iter()
        .fold(0, |acc, &neighbour| {
            match tiles.get(&neighbour) {
                Some(Color::Black) => acc + 1,
                _ => {
                    //tiles.insert(neighbour, Color::Black);
                    acc
                }
            }
        })
}

fn daily_flip(tiles: &mut HashMap<Tile, Color>) -> HashMap<Tile, Color> {
    let mut new_tiles = tiles.clone();

    for (&coordinate, color) in tiles.iter() {
        let adjacent_black = count_black_neighbours(coordinate, tiles);
        match color {
            Color::Black => {
                if adjacent_black == 0 || adjacent_black > 2 {
                    new_tiles.insert(coordinate, Color::White);
                }
            }
            Color::White => {
                if adjacent_black == 2 {
                    new_tiles.insert(coordinate, Color::Black);
                }
            }
        }
    }

    new_tiles
}

fn part2(tiles: &mut HashMap<Tile, Color>) {
    let mut copy = tiles.clone();

    let all_set = copy.clone();
    for &coordinate in all_set.keys() {
        for neighbour in neighbours(coordinate) {
            copy.entry(neighbour).or_insert(Color::White);
        }
    }

    for _ in 0..100 {
        let all_set = copy.clone();
        for (&coordinate, color) in all_set.iter() {
            match color {
                Color::Black => {
                    for neighbour in neighbours(coordinate) {
                        copy.entry(neighbour).or_insert(Color::White);
                    }
                }
                Color::White => continue,
            }
        }

        copy = daily_flip(&mut copy);
    }

    let total_black = copy.values()
        .filter(|color| {
            match color {
                Color::Black => true,
                Color::White => false,
            }
        })
        .count();

    println!("total {}", total_black);

}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    let instructions: Vec<Vec<String>> = parse(&lines);

    let mut finished = part1(&instructions);
    part2(&mut finished);

    Ok(())
}
