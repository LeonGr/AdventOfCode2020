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

fn part1(messages: &Vec<String>, re: Regex) {
    let total = messages
        .iter()
        .filter(|message| re.is_match(message))
        .count();

    println!("total {}", total);
}

fn part2(lines: &Vec<String>) {
}

fn build_regex(rules: &Vec<String>) -> Regex {

    let number_regex = Regex::new(r"\d").unwrap();

    let mut rules_list: Vec<String> = vec![String::new(); rules.len()];

    for rule in rules {
        let parts = rule.split(": ").collect::<Vec<&str>>();
        let index = parts[0].parse::<usize>().unwrap();
        rules_list[index] = format!("{} ", parts[1]);
    }

    let mut regex_string = String::new() + "^ " + &rules_list[0] + " ";

    while number_regex.is_match(regex_string.as_str()) {
        for i in 1..rules.len() {
            let number_string = format!(" {} ", i);
            let number_str = number_string.as_str();
            if regex_string.contains(number_str) {
                regex_string = regex_string.replace(number_str, format!(" ( {} ) ", rules_list[i]).as_str());
            }
        }
    }

    regex_string = regex_string.replace(" ", "").replace("\"", "") + "$";

    Regex::new(regex_string.as_str()).unwrap()
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

    let re = build_regex(&rules);

    part1(&messages, re);
    //part2(&lines);

    Ok(())
}
