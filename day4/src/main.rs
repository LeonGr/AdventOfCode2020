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
    let found_height = match height_re.find(&height[..]) {
        Some(hgt) => hgt.as_str(),
        _ => return false
    };

    if height.ends_with("cm") {
        if !(150..=193).contains(&found_height.parse::<u8>().unwrap()) {
            return false;
        }
    } else if height.ends_with("in") {
        if !(59..=76).contains(&found_height.parse::<u8>().unwrap()) {
            return false;
        }
    } else {
        return false;
    }

    let hair_color: String = match fields.get("hcl") {
        Some(hair_color) => hair_color.clone(),
        _ => return false
    };

    let hair_color_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    match hair_color_re.find(&hair_color[..]) {
        Some(hc) => hc.as_str(),
        _ => return false
    };

    let eye_color: String = match fields.get("ecl") {
        Some(eye_color) => eye_color.clone(),
        _ => return false
    };

    match eye_color.as_str() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
        _ => return false
    }

    let passport_id: String = match fields.get("pid") {
        Some(id) => id.clone(),
        _ => return false
    };

    let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
    match pid_re.find(&passport_id[..]) {
        Some(pid) => pid.as_str(),
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
