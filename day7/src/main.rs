use std::{io::BufRead, collections::HashMap};
use regex::Regex;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader.lines().filter_map(std::io::Result::ok).collect())
}

fn build_rules_map(input: &Vec<String>) -> HashMap<String, HashMap<String, u8>> {
    let mut bag_rules: HashMap<String, HashMap<String, u8>> = HashMap::new();

    let amount_re = Regex::new(r"^\d+").unwrap();
    let bag_re = Regex::new(r"\w+\s\w+\sbag").unwrap();

    for line in input {
        let parts = line.split(" bags contain ").collect::<Vec<&str>>();
        let bag = parts[0].to_string();
        let rule = parts[1];

        let mut bag_capacity: HashMap<String, u8> = HashMap::new();

        if !rule.starts_with("no") {
            let capacities = rule.split(", ").collect::<Vec<&str>>();

            capacities.iter().for_each(|bag_rule| {
                let amount = match amount_re.find(&bag_rule) {
                    Some(amount) => amount.as_str().parse::<u8>().unwrap(),
                    _ => panic!("No amount")
                };

                let bag = match bag_re.find(&bag_rule) {
                    Some(bag) => bag.as_str().replace(" bag", ""),
                    _ => panic!("No bag")
                };

                bag_capacity.insert(bag, amount);
            });
        }

        bag_rules.insert(bag, bag_capacity);
    }

    return bag_rules;
}

fn bag_can_contain_shiny_gold(bag: &String, rules: &HashMap<String, HashMap<String, u8>>) -> bool {
    let current_bag_rules = rules.get(bag).unwrap();

    return current_bag_rules
        .keys()
        .any(|nested_bag| {
            nested_bag == &"shiny gold".to_string() || bag_can_contain_shiny_gold(nested_bag, rules)
        });
}

fn part1(bag_rules: &HashMap<String, HashMap<String, u8>>) {
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
        return current_bag_rules
            .iter()
            .fold(1, |acc, (color, &amount)| {
                //println!("{:?}", (color, &amount));
                acc + (amount as u32 * count_bags(color, rules))
            })
    }
}

fn part2(bag_rules: &HashMap<String, HashMap<String, u8>>) {
    println!("total 2 {:?}", count_bags(&"shiny gold".to_string(), bag_rules) - 1);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    let rules = build_rules_map(&lines);

    part1(&rules);
    part2(&rules);

    Ok(())
}
