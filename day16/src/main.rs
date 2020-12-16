use std::ops::RangeInclusive;
use std::{ collections::{HashMap, HashSet}, io::BufRead };

type ValidRange = RangeInclusive<u16>;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn parse_fields_string(line: &String) -> Vec<u16> {
    line.split(",").map(|n| n.parse::<u16>().unwrap()).collect()
}

fn parse_input(lines: &Vec<String>) -> Parsed {
    let mut current_stage = 0;

    let mut fields: HashMap<String, (ValidRange, ValidRange)> = HashMap::new();
    let mut values: Vec<u16> = vec![];
    let mut nearby_tickets_values: Vec<Vec<u16>> = vec![];

    for line in lines {
        if line == "" {
            current_stage += 1;
            continue;
        } else if line == "your ticket:" || line == "nearby tickets:" {
            continue;
        }

        if current_stage == 0 {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let key = parts[0];

            let ranges = parts[1]
                .split(" or ")
                .map(|range| {
                    let numbers = range
                        .split("-")
                        .map(|n| n.parse::<u16>().unwrap())
                        .collect::<Vec<u16>>();
                    numbers[0]..=numbers[1]
                })
                .collect::<Vec<ValidRange>>();

            fields.insert(key.to_string(), (ranges[0].to_owned(), ranges[1].to_owned()));
        } else if current_stage == 1 {
            values = parse_fields_string(line);
        } else if current_stage == 2 {
            nearby_tickets_values.push(parse_fields_string(line));
        }
    }

    let nearby_tickets: Vec<Ticket> =
        nearby_tickets_values
            .into_iter()
            .fold(vec![], |mut acc, values| {
                acc.push(Ticket { values });
                acc
            });

    let my_ticket = Ticket { values };

    return Parsed { fields, my_ticket, nearby_tickets };
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u16>,
}

#[derive(Debug)]
struct Parsed {
    fields: HashMap<String, (ValidRange, ValidRange)>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn part1(parsed: &Parsed) {
    let ticket_scanning_error_rate = parsed.nearby_tickets.iter().fold(0, |acc, ticket| {
        acc + ticket.values.iter().fold(0, |total, value| {
            let valid_somewhere = parsed
                .fields
                .values()
                .any(|(range1, range2)| range1.contains(&value) || range2.contains(&value));

            if !valid_somewhere {
                total + value
            } else {
                total
            }
        })
    });

    println!("error rate {}", ticket_scanning_error_rate);
}

fn possible_fields(value: u16, fields: &HashMap<String, (ValidRange, ValidRange)>) -> HashSet<String> {
    fields
        .iter()
        .fold(HashSet::new(), |mut acc, (field_name, (range1, range2))| {
            if range1.contains(&value) || range2.contains(&value) {
                acc.insert(field_name.to_owned());
            }

            acc
        })
}

fn part2(parsed: &Parsed) {
    let mut valid_tickets: Vec<&Ticket> = parsed.nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.values.iter().all(|value| {
                parsed.fields
                    .values()
                    .any(|(range1, range2)| range1.contains(&value) || range2.contains(&value))
            })
        })
        .collect();

    valid_tickets.push(&parsed.my_ticket);

    let mut keys_to_find = parsed.fields
        .keys()
        .map(|x| x.to_owned())
        .collect::<HashSet<String>>();
    let mut ordered_fields = vec!["".to_string(); 20];
    let number_of_fields = keys_to_find.len();
    let mut fields_to_find = HashSet::new();

    for i in 0..number_of_fields {
        fields_to_find.insert(i);
    }

    for _ in 0..number_of_fields {
        for i in fields_to_find.clone() {
            let options = valid_tickets
                .iter()
                .fold(keys_to_find.clone(), |acc, ticket| {
                    acc.intersection(&possible_fields(ticket.values[i], &parsed.fields))
                        .map(|x| x.to_owned())
                        .collect()
                })
                .iter()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();

            if options.len() == 1 {
                keys_to_find.remove(&options[0]);
                ordered_fields[i] = options[0].clone();
                fields_to_find.remove(&i);
                break;
            }
        }
    }

    let mut total: u64 = 1;
    for i in 0..number_of_fields {
        if ordered_fields[i].starts_with("departure") {
            total *= parsed.my_ticket.values[i] as u64;
        }
    }

    println!("{:?}", total);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    let parsed = parse_input(&lines);

    part1(&parsed);
    part2(&parsed);

    Ok(())
}
