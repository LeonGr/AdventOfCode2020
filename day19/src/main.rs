use regex::Regex;
use std::{collections::HashMap, io::BufRead};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn build_regex_string(start_rule: String, rules_map: &HashMap<usize, String>) -> String {
    let mut regex_string = start_rule.clone();

    while Regex::new(r"\d").unwrap().is_match(regex_string.as_str()) {
        for i in rules_map.keys() {
            let number_string = format!(" {} ", i);
            let number_str = number_string.as_str();
            if regex_string.contains(number_str) {
                let current_rule = rules_map.get(&i).unwrap();
                regex_string = regex_string.replace(number_str, format!(" ( {} ) ", current_rule).as_str());
            }
        }
    }

    regex_string.replace(" ", "").replace("\"", "")
}

fn build_rules_map(rules: &Vec<String>) -> HashMap<usize, String> {
    let mut rules_map: HashMap<usize, String> = HashMap::new();

    for rule in rules {
        let parts = rule.split(": ").collect::<Vec<&str>>();
        let index = parts[0].parse::<usize>().unwrap();
        rules_map.insert(index, format!("{} ", parts[1]));
    }

    rules_map
}

fn _build_regex(rules: &Vec<String>) -> Regex {
    let number_regex = Regex::new(r"\d").unwrap();

    let mut rules_map: HashMap<usize, String> = HashMap::new();

    for rule in rules {
        let parts = rule.split(": ").collect::<Vec<&str>>();
        let index = parts[0].parse::<usize>().unwrap();
        rules_map.insert(index, format!("{} ", parts[1]));
    }

    let mut regex_string = String::new() + "^ " + rules_map.get(&(0 as usize)).unwrap() + " ";
    println!("{:?}", regex_string);

    while number_regex.is_match(regex_string.as_str()) {
        for i in rules_map.keys() {
            let number_string = format!(" {} ", i);
            let number_str = number_string.as_str();
            if regex_string.contains(number_str) {
                let current_rule = rules_map.get(&i).unwrap();
                regex_string = regex_string.replace(number_str, format!(" ( {} ) ", current_rule).as_str());
            }
        }
    }

    regex_string = regex_string.replace(" ", "").replace("\"", "") + "$";

    println!("{:?}", regex_string);

    Regex::new(regex_string.as_str()).unwrap()
}

fn part1(messages: &Vec<String>, re: Regex) {
    let total = messages
        .iter()
        .filter(|message| {
            re.is_match(message)
        })
        .count();

    println!("total {}", total);
}

fn part2(messages: &Vec<String>, re_42: &str, re_31: &str) {
    // Ugly but Rust has no recursive regex
    let mut re_string = "^(42)+(42(42(42(42(42(42(42(42(42(4231)?31)?31)?31)?31)?31)?31)?31)?31)?31)$".to_string();
    re_string = re_string.replace("42", re_42).replace("31", re_31);

    let re = Regex::new(re_string.as_str()).unwrap();

    let total = messages
        .iter()
        .filter(|message| {
            re.is_match(message)
        })
        .count();

    println!("total {}", total);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    let mut rules: Vec<String> = vec![];
    let mut messages: Vec<String> = vec![];
    let mut reading_rules = true;

    for line in lines {
        if line == "" {
            reading_rules = false;
        } else if reading_rules {
            rules.push(line);
        } else {
            messages.push(line);
        }
    }

    let rules_map = build_rules_map(&rules);
    let start_rule = String::new() + "^ " + rules_map.get(&(0 as usize)).unwrap() + " ";
    let regex_string = build_regex_string(start_rule, &rules_map) + "$";
    let re = Regex::new(regex_string.as_str()).unwrap();

    part1(&messages, re);

    let re_42 = build_regex_string(" 42 ".to_string(), &rules_map);
    let re_31 = build_regex_string(" 31 ".to_string(), &rules_map);

    part2(&messages, re_42.as_str(), re_31.as_str());

    Ok(())
}
