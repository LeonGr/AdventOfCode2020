use std::io::BufRead;
use std::fs::File;
use std::io::Read;

use std::collections::HashSet;
use std::iter::FromIterator;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader.lines().filter_map(std::io::Result::ok).collect())
}


fn read_input_string() -> std::io::Result<String> {
    let mut input = String::new();
    let mut f = File::open("input")?;
    f.read_to_string(&mut input).expect("Unable to read string");

    Ok(input)
}

fn part1(lines: &String) {
    let count: usize = lines
        .split("\n\n")
        .map(|group| {
            let mut char_list: Vec<char> = group
                    .chars()
                    .filter(|c| c != &'\n')
                    .collect();
            char_list.sort();
            char_list.dedup();
            char_list.len()
        })
        .sum();

    println!("{:?}", count);
}

fn part2(lines: &String) {
    let alphabet = ('a'..='z').collect::<HashSet<char>>();

    let count = lines
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|x| x.chars().collect::<HashSet<char>>())
                .collect::<Vec<HashSet<char>>>()
        })
        .fold(0, |acc, group| {
            acc + group
                .iter()
                .fold(alphabet.clone(), |acc, x| {
                    acc.intersection(&x)
                    .map(|c| *c)
                    .collect::<HashSet<char>>()
                })
                .len()
        });

    println!("{}", count);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_string()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
