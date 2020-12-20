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

            let rotated = {
                if c == 0 {
                    rotate(&puzzle[r-1][0], 2)
                } else {
                    rotate(&puzzle[r][c-1], 1)
                }
            };

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
                        if c != 0 {
                            new = rotate(&new, 1);
                        }
                        puzzle[r][c] = new;
                        tiles_left = tiles_left
                            .iter()
                            .filter(|t| t.id != tile.id)
                            .map(|tile| tile.to_owned())
                            .collect::<Vec<Tile>>();
                    }
                    None => continue,
                }
            }
        }
    }

    puzzle
}

fn create_monster_coordinates() -> Vec<(usize, usize)> {
    let monster = r"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

    println!("monster: \n{}", monster);
    let mut monster_coordinates: Vec<(usize, usize)> = vec![];
    let monster_lines = monster.split("\n").collect::<Vec<&str>>();
    for i in 0..monster_lines.len() {
        println!("{:?}", monster_lines[i]);
        let line = monster_lines[i].chars().collect::<Vec<char>>();
        for j in 0..line.len() {
            if line[j] == '#' {
                monster_coordinates.push((i, j));
            }
        }
    }

    monster_coordinates
}

fn find_sea_monsters(image: &Tile, monster_coordinates: &Vec<(usize, usize)>) -> Option<(u8, Tile)> {
    println!("{:?}", monster_coordinates);

    let mut pixels = image.pixels.clone();

    let monster_length = 20;
    let monster_height = 3;

    let mut monsters_found = 0;

    for r in 0..pixels.len() - monster_height {
        for c in 0..pixels[0].len() - monster_length {
            let mut monster_at_coordinate = true;
            for (x, y) in monster_coordinates {
                if pixels[r+x][c+y] != '#' && pixels[r+x][c+y] != 'O' {
                    monster_at_coordinate = false;
                }
            }

            if monster_at_coordinate {
                for (x, y) in monster_coordinates {
                    pixels[r+x][c+y] = 'O';
                }
                monsters_found += 1;
            }
        }
    }

    println!("found {} monsters", monsters_found);
    if monsters_found > 0 {
        Some((monsters_found, Tile { id: 2, pixels }))
    } else {
        None
    }
}

fn part2(tiles: &Vec<Tile>) {
    let puzzle = finish_puzzle(tiles);
    println!("{:?}", puzzle);

    let puzzle_size = (tiles.len() as f32).sqrt() as usize;
    let piece_size = tiles[0].pixels.len();
    let empty_tile = Tile { id: 0, pixels: vec![vec!['.'; piece_size]; piece_size] };
    let mut no_border_puzzle = vec![vec![empty_tile.clone(); puzzle_size]; puzzle_size];

    for r in 0..puzzle_size {
        for c in 0..puzzle_size {
            let piece = &puzzle[r][c];
            let mut new_pixels: Vec<Vec<char>> = vec![];
            for i in 1..piece.pixels.len() - 1 {
                let relevant_pixels = &piece.pixels[i][1..piece_size - 1];
                println!("relevant_pixels {:?}", relevant_pixels);
                new_pixels.push(relevant_pixels.to_vec());
            }

            no_border_puzzle[r][c] = Tile { id: piece.id, pixels: new_pixels };
        }
    }

    println!("{:?}", no_border_puzzle);

    let mut puzzle_string = String::new();
    for r in 0..puzzle_size {
        for i in 0..piece_size - 2 {
            for c in 0..puzzle_size {
                puzzle_string += no_border_puzzle[r][c].pixels[i].iter().collect::<String>().as_str();
            }
            puzzle_string += "\n";
        }
    }

    println!("{}", puzzle_string);

    let mut current_pixels = vec![];

    for line in puzzle_string.split("\n") {
        current_pixels.push(line.chars().collect::<Vec<char>>());
    }
    current_pixels.pop();

    let puzzle_tile = Tile { id: 1, pixels: current_pixels };
    println!("{:?}", rotate(&flip(&puzzle_tile), 1));
    let monster_coordinates = create_monster_coordinates();
    find_sea_monsters(&rotate(&flip(&puzzle_tile), 1), &monster_coordinates);

    for a in 0..=3 {
        let rotated = rotate(&puzzle_tile, a);
        match find_sea_monsters(&rotated, &monster_coordinates) {
            Some((num, tile)) => {
                println!("{} {:?}", num, tile);
                let mut total_hash = 0;
                for row in tile.pixels {
                    for chr in row {
                        if chr == '#' {
                            total_hash += 1;
                        }
                    }
                }
                println!("roughness {}", total_hash);
                break;
            }
            None => (),
        }
        let flipped = flip(&rotated);
        match find_sea_monsters(&flipped, &monster_coordinates) {
            Some((num, tile)) => {
                println!("{} {:?}", num, tile);
                let mut total_hash = 0;
                for row in tile.pixels {
                    for chr in row {
                        if chr == '#' {
                            total_hash += 1;
                        }
                    }
                }
                println!("roughness {}", total_hash);
                break;
            }
            None => (),
        }
    }
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    let tiles = parse(&lines);

    //part1(&tiles);
    part2(&tiles);

    Ok(())
}
