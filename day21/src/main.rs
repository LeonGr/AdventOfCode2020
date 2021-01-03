use std::{io::BufRead, collections::{HashMap, HashSet}};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn part1(lines: &Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut allergen_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut cant_possibly: HashSet<String> = HashSet::new();
    let mut occurrences: HashMap<String, u32> = HashMap::new();

    for line in lines {
        let mut copy = line.to_owned();
        copy.pop();
        let parts = copy.split(" (contains ").collect::<Vec<&str>>();
        let ingredients = parts[0].split(" ").collect::<Vec<&str>>();
        let allergens = parts[1].split(", ").collect::<Vec<&str>>();

        for allergen in &allergens {
            let ingredient_set: HashSet<String> = ingredients.iter().map(|x| x.to_string()).collect::<Vec<String>>().iter().cloned().collect();
            println!("allergen {}, ingredient_set {:?}", allergen, ingredient_set);
            let set = allergen_map.entry(allergen.to_string()).or_insert(ingredient_set.clone());
            println!("set {:?}", set);
            let intersection: HashSet<String> = set.intersection(&ingredient_set).map(|x| x.to_owned()).collect();
            println!("intersection {:?}", intersection);

            allergen_map.insert(allergen.to_string(), intersection);
        }

        for ingredient in &ingredients {
            let number: u32 = match occurrences.get(&ingredient.to_string()) {
                Some(number) => *number,
                None => 0,
            };
            occurrences.insert(ingredient.to_string(), number + 1);
            cant_possibly.insert(ingredient.to_string());
        }

        println!("{:?} {:?}", ingredients, allergens);
    }

    println!("\n{:?}", allergen_map);
    for set in allergen_map.values() {
        let removed: HashSet<String> = cant_possibly.difference(set).map(|x| x.to_owned()).collect();
        cant_possibly = removed;
    }

    println!("cant_possibly {:?}", cant_possibly);
    let mut sum = 0;
    for (key, value) in occurrences.iter() {
        if cant_possibly.contains(key) {
            sum += value;
        }
    }

    println!("total {}\n", sum);
    allergen_map
}

fn part2(allergen_map: &HashMap<String, HashSet<String>>) {
    println!("{:?}", allergen_map);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    let allergen_map = part1(&lines);
    part2(&allergen_map);

    Ok(())
}
