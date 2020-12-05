use std::{io::BufRead, collections::HashMap};
use regex::Regex;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader.lines().filter_map(std::io::Result::ok).collect())
}

fn part1(lines: &Vec<String>) {
    let mut copy_lines = lines.clone();
    copy_lines.push("".to_string());

    let mut num_valid = 0;
    let mut has_cid = false;
    let mut num_fields = 0;

    for line in copy_lines {
        if line == "" {
            let is_valid = num_fields == 8 || (num_fields == 7 && !has_cid);
            if is_valid {
                num_valid += 1;
            }

            has_cid = false;
            num_fields = 0;
        } else {
            num_fields += line.matches(":").count();
            has_cid = line.contains("cid") || has_cid;
        }
    }

    println!("number of valid passports: {}", num_valid);
}

fn check_passport(fields: HashMap<String, String>) -> bool {
    match fields.get("byr") {
        Some(year) if (1920..=2002).contains(&year.parse::<u16>().unwrap()) => (),
        _ => return false
    }

    match fields.get("iyr") {
        Some(year) if (2010..=2020).contains(&year.parse::<u16>().unwrap()) => (),
        _ => return false
    }

    match fields.get("eyr") {
        Some(year) if (2020..=2030).contains(&year.parse::<u16>().unwrap()) => (),
        _ => return false
    }

    let height: String = match fields.get("hgt") {
        Some(height) => height.clone(),
        _ => return false
    };

    let height_re = Regex::new(r"^\d*").unwrap();
    let height_value = match height_re.find(&height[..]) {
        Some(hgt) => hgt.as_str(),
        _ => return false
    };

    if height.ends_with("cm") {
        if !(150..=193).contains(&height_value.parse::<u8>().unwrap()) {
            return false;
        }
    } else if height.ends_with("in") {
        if !(59..=76).contains(&height_value.parse::<u8>().unwrap()) {
            return false;
        }
    } else {
        return false;
    }

    match fields.get("hcl") {
        Some(hcl) if Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(hcl) => (),
        _ => return false
    };

    match fields.get("ecl") {
        Some(ecl) if Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap().is_match(ecl) => (),
        _ => return false
    };

    match fields.get("pid") {
        Some(pid) if Regex::new(r"^[0-9]{9}$").unwrap().is_match(pid) => (),
        _ => return false
    };

    true
}

fn part2(lines: &Vec<String>) {
    let mut copy_lines = lines.clone();
    copy_lines.push("".to_string());

    let mut num_valid = 0;
    let mut has_cid = false;
    let mut num_fields = 0;

    let mut passport_fields: HashMap<String, String> = HashMap::new();

    for line in copy_lines {
        if line == "" {
            let is_valid = (num_fields == 8 || (num_fields == 7 && !has_cid)) && check_passport(passport_fields);

            if is_valid {
                num_valid += 1;
            }

            has_cid = false;
            num_fields = 0;
            passport_fields = HashMap::new();
        } else {
            let fields: Vec<&str> = line.split(" ").collect();

            for field in fields {
                let field_value = field.split(":").collect::<Vec<&str>>();
                passport_fields.insert(field_value[0].to_string(), field_value[1].to_string());
            }

            num_fields += line.matches(":").count();
            has_cid = line.contains("cid") || has_cid;
        }
    }

    println!("number of valid passports: {}", num_valid);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
