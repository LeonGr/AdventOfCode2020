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

fn build_regex(rules: &Vec<String>) -> Regex {
    let number_regex = Regex::new(r"\d").unwrap();

    //let mut rules_list: Vec<String> = vec![String::new(); rules.len() * 2];
    let mut rules_map: HashMap<usize, String> = HashMap::new();

    for rule in rules {
        let parts = rule.split(": ").collect::<Vec<&str>>();
        let index = parts[0].parse::<usize>().unwrap();
        //rules_list[index] = format!("{} ", parts[1]);
        rules_map.insert(index, format!("{} ", parts[1]));
    }

    //let mut regex_string = String::new() + "^ " + &rules_list[0] + " ";
    let mut regex_string = String::new() + "^ " + rules_map.get(&(0 as usize)).unwrap() + " ";
    println!("{:?}", regex_string);
    //regex_string = "^ 8 # 11 ".to_string();
    regex_string = "^ 42 ".to_string();

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
            if re.is_match(message) {
                println!("{}", message);
                return true;
            }
            false
        })
        .count();

    println!("total {}", total);
}

fn part2(messages: &Vec<String>) {
    //let re_string = "^(((b)((a)((b)(b)|(a)(b))|(b)(((a)|(b))((a)|(b))))|(a)((b)((b)(b))|(a)((b)(b)|(a)((a)|(b)))))(b)|((((a)(a)|(a)(b))(a)|((b)(b))(b))(b)|((((a)|(b))(a)|(b)(b))(a))(a))(a)){2,}((b)((b)((a)((b)(a))|(b)((a)(a)))|(a)((b)((a)(b)|((a)|(b))(a))|(a)((b)(a)|(a)(b))))|(a)((b)(((a)(b)|((a)|(b))(a))(b)|(((a)|(b))(a)|(b)(b))(a))|(a)(((b)(a))(b)|((b)(a)|(b)(b))(a))))+";
    //let re_42 = "(((b)((a)((b)(b)|(a)(b))|(b)(((a)|(b))((a)|(b))))|(a)((b)((b)(b))|(a)((b)(b)|(a)((a)|(b)))))(b)|((((a)(a)|(a)(b))(a)|((b)(b))(b))(b)|((((a)|(b))(a)|(b)(b))(a))(a))(a))";
    //let re_31 = "((b)((b)((a)((b)(a))|(b)((a)(a)))|(a)((b)((a)(b)|((a)|(b))(a))|(a)((b)(a)|(a)(b))))|(a)((b)(((a)(b)|((a)|(b))(a))(b)|(((a)|(b))(a)|(b)(b))(a))|(a)(((b)(a))(b)|((b)(a)|(b)(b))(a))))";
    let re_42 = "((a)((((b)((a)((a)((b)(a)|(b)(b))|(b)((b)(a)|(a)(a)))|(b)(((b)((b)|(a))|(a)(a))(a)|((b)(b))(b)))|(a)((a)((b)((b)(a))|(a)((b)(a)|(b)(b)))|(b)((b)((b)(a)|(a)(b))|(a)((b)(b)))))(a)|((((b)((b)((b)|(a))|(a)(a))|(a)((a)(a)|(a)(b)))(b)|(((a)(a))(b)|(((b)|(a))((b)|(a)))(a))(a))(b)|(((((b)|(a))((b)|(a)))(b))(a)|((((b)|(a))((b)|(a)))(b)|((b)((b)|(a))|(a)(a))(a))(b))(a))(b))(a)|((a)((((a)((a)(a))|(b)((a)(b)))(a)|((b)((b)(a))|(a)((b)(b)|(a)(a)))(b))(b)|((b)((b)((b)(a)|(a)(a))|(a)((b)((b)|(a))|(a)(a)))|(a)(((a)(b))(a)|((b)(a)|(a)(a))(b)))(a))|(b)((b)((b)((b)((b)(a)|(a)(a))|(a)((b)(a)|(a)(b)))|(a)((a)((b)(b))|(b)((b)(a)|(a)(a))))|(a)((b)((b)((b)(a)|(a)(a))|(a)((b)(a)|(a)(b)))|(a)((b)((a)(a))|(a)((b)(a)|(a)(a))))))(b))|(b)((a)((b)((((a)((b)(a)|(a)(b))|(b)((b)(b)|(a)(a)))(a)|((a)((b)((b)|(a))|(a)(a))|(b)((a)(a)|(a)(b)))(b))(a)|((b)((b)((b)(a)|((b)|(a))(b))|(a)((a)(b)))|(a)((a)((b)((b)|(a))|(a)(a))|(b)((b)(b))))(b))|(a)(((a)((b)(((b)|(a))((b)|(a)))|(a)((a)(b)))|(b)((a)((b)(a))|(b)((b)(a)|(b)(b))))(a)|(((a)((b)(a)|(b)(b))|(b)((b)(a)|(a)(b)))(b)|((b)((b)(a)|(a)(a))|(a)((b)(a)|((b)|(a))(b)))(a))(b)))|(b)((a)((a)(((a)((b)(b)|(a)(b))|(b)((b)((b)|(a))|(a)(a)))(a)|((b)((b)(a)|(a)(b))|(a)((a)(a)|(a)(b)))(b))|(b)((b)(((b)(b)|(a)(a))(b)|((b)(a)|((b)|(a))(b))(a))|(a)((a)((b)((b)|(a))|(a)(a))|(b)((b)(b)))))|(b)((a)((b)(((a)(a)|(a)(b))(a)|((b)(a)|(a)(a))(b))|(a)((((b)|(a))((b)|(a)))(b)|((b)(a))(a)))|(b)((((b)(a)|(b)(b))(a)|((b)(b))(b))(b)|((b)((a)(a))|(a)((b)(a)))(a))))))";
    let re_31 = "((a)(((a)((b)(((b)((b)(b)|(a)(b))|(a)((b)((b)|(a))|(a)(a)))(b)|(((b)(a)|(a)((b)|(a)))(a)|((b)(b)|(a)(a))(b))(a))|(a)((((b)((b)|(a))|(a)(a))(b)|((b)(a)|(a)(a))(a))(a)|((a)((a)(a)))(b)))|(b)((a)((b)(((b)(b)|(a)(b))((b)|(a)))|(a)(((a)(a)|(a)(b))(a)|((a)(a))(b)))|(b)(((b)((a)(b))|(a)((b)(a)|((b)|(a))(b)))(a)|((a)((a)(a))|(b)((a)(b)))(b))))(a)|((((b)((a)(((b)|(a))((b)|(a)))|(b)((b)(a)|((b)|(a))(b)))|(a)(((b)((b)|(a))|(a)(a))(b)|((b)(a)|(a)(b))(a)))(b)|(((b)((b)(a)|(a)(a))|(a)((b)(a)|((b)|(a))(b)))(a)|((((b)|(a))((b)|(a)))(b)|((b)((b)|(a))|(a)(a))(a))(b))(a))(b)|(((a)(((a)(a)|(a)(b))(a)|((a)(b))(b))|(b)((a)((b)(a))|(b)((b)(a)|(a)(a))))(a)|((b)((a)((a)(b))|(b)((a)(a)))|(a)((a)((b)(b)|(a)(b))|(b)((b)((b)|(a))|(a)(a))))(b))(a))(b))|(b)(((b)(((b)(((b)(b)|(a)(b))(b)|((b)(a)|(a)(a))(a))|(a)((b)((b)(a)|(a)(b))|(a)((a)(a)|(a)(b))))(a)|((a)(((a)(b))(a)|((b)(a))(b))|(b)((((b)|(a))((b)|(a)))(b)|((b)((b)|(a))|(a)(a))(a)))(b))|(a)((b)((a)((b)((b)(a))|(a)((b)(b)|(a)(a)))|(b)(((b)(a)|(a)(b))(a)|((b)(a)|(b)(b))(b)))|(a)((a)(((a)(a)|(a)(b))(a)|((a)(b))(b))|(b)(((b)(a)|(a)((b)|(a)))(b)|((b)(a)|(a)(a))(a)))))(a)|((b)(((a)(((b)((b)|(a))|(a)(a))(b)|((b)(a)|(a)(b))(a))|(b)(((b)(a))(b)|((b)(a))(a)))(a)|(((a)((b)(b))|(b)((b)(a)|((b)|(a))(b)))(a)|((b)((b)(a)|(a)(b))|(a)((a)(a)|(a)(b)))(b))(b))|(a)((b)((((b)(a)|((b)|(a))(b))(a)|((b)(a)|(a)(b))(b))(a)|(((a)(a)|(a)(b))(a)|((b)(a))(b))(b))|(a)((((b)(a)|(a)(a))(a)|((b)(b)|(a)(a))(b))(b)|((a)((a)(a))|(b)((b)(b)|(a)(b)))(a))))(b)))";
    let re_string = format!(r"^{}{{2,}}{}+", re_42, re_31);
    let re = Regex::new(re_string.as_str()).unwrap();

    let total = messages
        .iter()
        .filter(|message| {
            if re.is_match(message) {
                println!("{}", message);
                let left = format!("{}", re_42);
                let right = format!("{}", re_31);
                //let without_left = Regex::new(format!("^{}{{2,}}", left).as_str()).unwrap().replace(message, "").into_owned();
                //println!("without_left {}", without_left);
                let without_right = Regex::new(format!("{}+$", right).as_str()).unwrap().replace(message, "").into_owned();
                let num_left = Regex::new(left.as_str()).unwrap().find_iter(without_right.as_str()).count();
                println!("without_right {}", without_right);
                let num_right = Regex::new(right.as_str()).unwrap().find_iter(message).count();
                println!("left {} right {}", num_left, num_right);
                //return  num_left >= num_right;
                return num_left >= num_right;
            }
            false
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

    let re = build_regex(&rules);

    part1(&messages, re);
    part2(&messages);

    Ok(())
}
