use regex::Regex;
use std::{collections::{HashMap, HashSet}, io::BufRead};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn build_rules_map(input: &Vec<String>) -> (HashMap<String, HashMap<String, u8>>, HashMap<String, HashSet<String>>) {
    let mut bag_rules: HashMap<String, HashMap<String, u8>> = HashMap::new();

    let mut bag_can_be_contained_in: HashMap<String, HashSet<String>> = HashMap::new();

    let amount_re = Regex::new(r"^\d+").unwrap();
    let bag_re = Regex::new(r"\w+\s\w+\sbag").unwrap();

    for line in input {
        let parts = line.split(" bags contain ").collect::<Vec<&str>>();
        let container = parts[0].to_string();
        let rule = parts[1];

        let mut bag_capacity: HashMap<String, u8> = HashMap::new();

        if !rule.starts_with("no") {
            rule.split(", ").for_each(|bag_rule| {
                let amount = match amount_re.find(&bag_rule) {
                    Some(amount) => amount.as_str().parse::<u8>().unwrap(),
                    _ => panic!("No amount"),
                };

                let contained = match bag_re.find(&bag_rule) {
                    Some(bag) => bag.as_str().replace(" bag", ""),
                    _ => panic!("No bag"),
                };

                let set = bag_can_be_contained_in.entry(contained.clone()).or_insert(HashSet::new());
                set.insert(container.clone());

                bag_capacity.insert(contained, amount);
            });
        }

        bag_rules.insert(container, bag_capacity);
    }

    return (bag_rules, bag_can_be_contained_in);
}

fn bag_can_contain_shiny_gold(bag: &String, rules: &HashMap<String, HashSet<String>>) -> bool {
    let current_bag_rules = rules.get("shiny gold").unwrap();

    if current_bag_rules.len() == 0 {
        return false;
    } else if current_bag_rules.contains(bag) {
        return true;
    } else {
        return current_bag_rules
            .iter()
            .any(|bag| {
                bag_can_contain_shiny_gold(bag, rules)
            })
    }
}

fn part1(bag_rules: &HashMap<String, HashSet<String>>) {
    let total = bag_rules
        .keys()
        .filter(|bag| bag_can_contain_shiny_gold(bag, &bag_rules))
        .count();

    println!("total 1 {}", total);
}

fn count_bags(bag: &String, rules: &HashMap<String, HashMap<String, u8>>) -> u32 {
    let current_bag_rules = rules.get(bag).unwrap();

    if current_bag_rules.keys().count() == 0 {
        return 1;
    } else {
        return current_bag_rules.iter().fold(1, |acc, (color, &amount)| {
            acc + (amount as u32 * count_bags(color, rules))
        });
    }
}

fn part2(bag_rules: &HashMap<String, HashMap<String, u8>>) {
    println!(
        "total 2 {:?}",
        count_bags(&"shiny gold".to_string(), bag_rules) - 1
    );
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    let (rules, can_contain_rules) = build_rules_map(&lines);

    part1(&can_contain_rules);
    part2(&rules);

    Ok(())
}
