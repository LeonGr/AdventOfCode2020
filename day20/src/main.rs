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
fn check_fit(tile1: &Tile, tile2: &Tile) -> bool {
    //println!("Checking {} with {}", tile1.id, tile2.id);
    let target = &tile1.pixels[0];
    //println!("checking for {:?}", target);

    for a in 0..=3 {
        let rotated = rotate(tile2, a);
        //println!("check        {:?}", rotated.pixels[0]);
        if rotated.pixels[0] == *target {
            //println!("Match: rotated by {}", a);
            return true;
        }
        let flipped = flip(&rotated);
        //println!("check        {:?}", flipped.pixels[0]);
        if flipped.pixels[0] == *target {
            //println!("Match: rotated by {} and flipped", a);
            return true;
        }
    }

    return false;
}

fn part1(tiles: &Vec<Tile>) {
    let mut corner_ids: Vec<u16> = vec![];

    for tile in tiles {
        let mut unfittable_sides = 0;

        for a in 0..=3 {
            let rotated = rotate(tile, a);

            let mut fits_somewhere = false;
            for other in tiles {
                if tile.id != other.id {
                    if check_fit(&rotated, other) {
                        fits_somewhere = true;
                    }
                }
            }

            if !fits_somewhere {
                unfittable_sides += 1;
            }
        }

        if unfittable_sides == 2 {
            println!("tile {} is a corner", tile.id);
            corner_ids.push(tile.id);
        }
    }

    let product = corner_ids.iter().map(|x| *x as u64).product::<u64>();
    println!("product {}", product);
}

fn part2(tiles: &Vec<Tile>) {
    println!("#tiles {}", tiles.len());
    //let puzzle = 
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    let tiles = parse(&lines);

    part1(&tiles);
    part2(&tiles);

    Ok(())
}
