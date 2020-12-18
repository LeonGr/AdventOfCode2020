use std::{ io::BufRead, collections::HashMap };

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

#[derive(Debug, Clone)]
enum Expr {
    Const(u64),
    Plus(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

fn evaluate(expression: Expr) -> u64 {
    match expression {
        Expr::Const(c) => c,
        Expr::Plus(a, b) => evaluate(Box::leak(a).clone()) + evaluate(Box::leak(b).clone()),
        Expr::Mult(a, b) => evaluate(Box::leak(a).clone()) * evaluate(Box::leak(b).clone()),
    }
}

fn find_index_matching_paren(expression: &String) -> usize {
    let mut open = 1;

    for (i, x) in expression[1..].chars().enumerate() {
        if x == '(' {
            open += 1;
        } else if x == ')' {
            open -= 1;
        }

        if open == 0 {
            return i + 1;
        }
    }

    unreachable!()
}

fn flip(expression: &String) -> String {
    expression.chars().rev().collect::<String>().replace(")", "x").replace("(", ")").replace("x", "(")
}

fn parse_left_to_right(expression: &String) -> Expr {
    let stripped = expression.split(" ").collect::<String>();
    let length = stripped.len();

    if length == 1 {
        return Expr::Const(stripped.parse::<u64>().unwrap());
    }

    if stripped.starts_with("(") && find_index_matching_paren(&stripped) == length - 1 {
        return parse_left_to_right(&stripped[1..(length - 1)].to_string());
    }

    let flipped: String = flip(&stripped);

    let mut indices = HashMap::new();

    for c in ['+', '*', '('].iter() {
        indices.insert(c, match flipped.find(*c) {
            Some(index) => index,
            None => length,
        });
    }

    let first_operator_index = {
        let index_paren = indices.get(&'(').unwrap();
        if  index_paren < indices.get(&'+').unwrap() && index_paren < indices.get(&'*').unwrap() {
            find_index_matching_paren(&flipped) + 1
        } else {
            1
        }
    };

    let first_operator = flipped.chars().nth(first_operator_index).unwrap();
    let first_part = flip(&flipped[0..first_operator_index].to_string());
    let second_part = flip(&flipped[(first_operator_index + 1)..].to_string());

    match first_operator {
        '+' => Expr::Plus(Box::new(parse_left_to_right(&first_part)), Box::new(parse_left_to_right(&second_part))),
        '*' => Expr::Mult(Box::new(parse_left_to_right(&first_part)), Box::new(parse_left_to_right(&second_part))),
        _ => unreachable!(),
    }
}

fn part1(lines: &Vec<String>) {
    let sum = lines
        .iter()
        .fold(0, |acc, line| {
            acc + evaluate(parse_left_to_right(line))
        });

    println!("part 1 sum: {}", sum);
}

fn insert_bracket(input: String) -> String {
    if input.starts_with("(") {
        let index_closing = find_index_matching_paren(&input);
        String::new() + &input[..=index_closing] + ")" + &input[(index_closing + 1)..]
    } else {
        String::new() + &input[..1] + ")" + &input[1..]
    }
}

fn parse_addition_priority(expression: &String) -> Expr {
    let mut stripped = expression.split(" ").collect::<String>();

    while stripped.contains("+") {
        let first_plus_index = match stripped.find('+') {
            Some(index) => index,
            None => return parse_left_to_right(&stripped),
        };

        let before_plus = flip(&insert_bracket(flip(&stripped[..first_plus_index].to_string())));
        let after_plus = insert_bracket(stripped[(first_plus_index + 1)..].to_string());

        stripped = before_plus + "#" + after_plus.as_str();
    }

    parse_left_to_right(&stripped.replace("#", "+"))
}

fn part2(lines: &Vec<String>) {
    let sum = lines
        .iter()
        .fold(0, |acc, line| {
            acc + evaluate(parse_addition_priority(line))
        });

    println!("part 2 sum: {}", sum);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    Ok(())
}
