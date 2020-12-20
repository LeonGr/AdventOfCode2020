use std::io::BufRead;
use regex::Regex;
use std::fmt;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

#[derive(Clone)]
struct Tile {
    id: u16,
    pixels: Vec<Vec<char>>,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = format!("\nTile {}\n", self.id);

        for row_pixels in &self.pixels {
            let as_string = row_pixels.iter().collect::<String>();
            output += format!("{}\n", as_string).as_str();
        }

        write!(f, "{}", output)
    }
}

fn parse(lines: &Vec<String>) -> Vec<Tile> {
    let number_regex = Regex::new(r"\d+").unwrap();

    let mut tiles: Vec<Tile> = vec![];

    let mut current_tile_id: u16 = 0;
    let mut current_pixels = vec![];

    for line in lines {
        if line == "" {
            if current_tile_id == 0 {
                panic!("Tile ID should be set");
            }
            tiles.push(Tile { id: current_tile_id.clone(), pixels: current_pixels.clone() });
            current_tile_id = 0;
            current_pixels = vec![];
        } else if line.starts_with("Tile ") {
            current_tile_id = number_regex.find(line).unwrap().as_str().parse::<u16>().unwrap();
        } else {
            current_pixels.push(line.chars().collect::<Vec<char>>());
        }
    }

    tiles
}

fn check_for_duplicate_borders(tiles: &Vec<Tile>) {
    for tile in tiles {
        for a in 0..=3 {
            let rotated = rotate(tile, a);

            for other in tiles {
                if tile.id != other.id {
                    if rotated.pixels[0] == other.pixels[0] {
                        println!("tile {} and {} have a duplicate border", tile.id, other.id);
                    }
                }
            }
        }
    }
}

// Rotate tile counterclockwise
fn rotate(tile: &Tile, angle: u8) -> Tile {
    let size = tile.pixels.len();
    let mut new_pixels = tile.pixels.clone();

    if angle == 0 {
        Tile { id: tile.id, pixels: new_pixels }
    } else {
        for r in 0..size {
            for c in 0..size {
                new_pixels[size-c-1][r] = tile.pixels[r][c];
            }
        }

        return rotate(&Tile { id: tile.id, pixels: new_pixels }, angle - 1);
    }
}

// Mirror tile vertically
fn flip(tile: &Tile) -> Tile {
    let size = tile.pixels.len();
    let mut new_pixels = vec![vec!['.'; size]; size];

    for r in 0..size {
        for c in 0..size {
            new_pixels[r][size-c-1] = tile.pixels[r][c];
        }
    }

    Tile { id: tile.id, pixels: new_pixels }
}

// Check if the top row of tile1 matches with any side of tile2
// Returns: Option<(rotation, if_flipped)>
fn check_fit(tile1: &Tile, tile2: &Tile) -> Option<(u8, bool)> {
    if tile1.id == tile2.id {
        panic!("Comparing the same tile");
    }

    let target = &tile1.pixels[0];

    for a in 0..=3 {
        let rotated = rotate(tile2, a);
        if rotated.pixels[0] == *target {
            return Some((a, false));
        }
        let flipped = flip(&rotated);
        if flipped.pixels[0] == *target {
            return Some((a, true));
        }
    }

    None
}

fn is_corner(tile: &Tile, tiles: &Vec<Tile>) -> Option<Vec<u8>> {
    let mut num_unfittable_sides = 0;
    let mut unfittable_rotations: Vec<u8> = vec![];

    for a in 0..=3 {
        let rotated = rotate(tile, a);

        let mut fits_somewhere = false;
        for other in tiles {
            if tile.id != other.id {
                match check_fit(&rotated, other) {
                    Some(_) => fits_somewhere = true,
                    None => continue,
                }
            }
        }

        if !fits_somewhere {
            num_unfittable_sides += 1;
            unfittable_rotations.push(a);
        }
    }


    if num_unfittable_sides == 2 {
        Some(unfittable_rotations)
    } else {
        None
    }
}

// Check if top (side 0) does not fit on anything
fn is_border_side(tile: &Tile, tiles: &Vec<Tile>) -> bool {
    let mut fits_somewhere = false;

    for other in tiles {
        if tile.id != other.id {
            match check_fit(&tile, other) {
                Some(_) => fits_somewhere = true,
                None => continue,
            }
        }
    }

    !fits_somewhere
}

fn part1(tiles: &Vec<Tile>) {
    let mut corner_ids: Vec<u16> = vec![];

    for tile in tiles {
        match is_corner(tile, tiles) {
            Some(_) => {
                println!("tile {} is a corner", tile.id);
                corner_ids.push(tile.id);
            }
            None => (),
        }
    }

    let product = corner_ids.iter().map(|x| *x as u64).product::<u64>();
    println!("product {}", product);
}

fn finish_puzzle(tiles: &Vec<Tile>) -> Vec<Vec<Tile>> {
    let puzzle_size = (tiles.len() as f32).sqrt() as usize;
    let piece_size = tiles[0].pixels.len();
    let empty_tile = Tile { id: 0, pixels: vec![vec!['.'; piece_size]; piece_size] };
    let mut puzzle = vec![vec![empty_tile.clone(); puzzle_size]; puzzle_size];

    // Find a corner piece and its sides that don't fit
    let mut initial_corner: Tile = empty_tile.clone();
    let mut rotation_top_unfittable: Vec<u8> = vec![];
    for tile in tiles {
        match is_corner(tile, tiles) {
            Some(status) => {
                initial_corner = tile.to_owned();
                rotation_top_unfittable = status;
                break;
            }
            None => continue,
        }
    }

    if initial_corner.id == 0 {
        unreachable!();
    }

    let mut initial_edge: Tile = empty_tile.clone();
    for tile in tiles {
        if initial_corner.id == tile.id {
            continue;
        }

        for a in 0..=(3 as u8) {
            if !rotation_top_unfittable.contains(&a) {
                let rotated = rotate(&initial_corner, a);
                match check_fit(&rotated, tile) {
                    Some(transform) => {
                        initial_edge = tile.to_owned();
                    }
                    None => continue,
                }
            }
        }

        if initial_edge.id != 0 {
            break;
        }
    }

    if initial_edge.id == 0 {
        unreachable!();
    }

    for a in 0..=3 {
        let rotated = rotate(&initial_edge, a);
        if is_border_side(&rotated, tiles) {
            match check_fit(&rotate(&rotated, 3), &initial_corner) {
                Some(_) => {
                    puzzle[0][1] = rotated.to_owned();
                },
                None => {
                    puzzle[0][1] = flip(&rotated);
                },
            }
        }
    }

    match check_fit(&rotate(&puzzle[0][1], 3), &initial_corner) {
        Some(transform) => {
            let mut new = rotate(&initial_corner, transform.0);
            if !transform.1 {
                new = flip(&new);
            }
            new = rotate(&new, 3);
            puzzle[0][0] = new;
        }
        None => unreachable!(),
    }


    let mut tiles_left = tiles
        .iter()
        .filter(|tile| ![initial_corner.id, initial_edge.id].contains(&tile.id))
        .map(|tile| tile.to_owned())
        .collect::<Vec<Tile>>();

    for r in 0..puzzle_size {
        for c in 0..puzzle_size {
            if puzzle[r][c].id != 0 {
                continue;
            }

            if c == 0 {
                let rotated = rotate(&puzzle[r-1][0], 2);
                for tile in tiles_left.clone() {
                    if rotated.id == tile.id {
                        continue;
                    }
                    match check_fit(&rotated, &tile) {
                        Some(transform) => {
                            let mut new = rotate(&tile, transform.0);
                            if !transform.1 {
                                new = flip(&new);
                            }
                            puzzle[r][c] = new;
                        }
                        None => continue,
                    }
                }
            }
            else {
                let rotated = rotate(&puzzle[r][c-1], 1);
                for tile in tiles_left.clone() {
                    if rotated.id == tile.id {
                        continue;
                    }
                    match check_fit(&rotated, &tile) {
                        Some(transform) => {
                            let mut new = rotate(&tile, transform.0);
                            if !transform.1 {
                                new = flip(&new);
                            }
                            new = rotate(&new, 1);
                            puzzle[r][c] = new;
                        }
                        None => continue,
                    }
                }
            }
        }
    }

    puzzle
}

fn part2(tiles: &Vec<Tile>) {
    let puzzle = finish_puzzle(tiles);
    println!("{:?}", puzzle);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    let tiles = parse(&lines);

    //part1(&tiles);
    part2(&tiles);

    Ok(())
}
