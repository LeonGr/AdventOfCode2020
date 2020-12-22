use std::io::BufRead;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn parse(lines: &Vec<String>) -> (Vec<u8>, Vec<u8>) {
    let mut parsing_p2 = false;

    let mut cards_p1: Vec<u8> = vec![];
    let mut cards_p2: Vec<u8> = vec![];

    for line in lines {
        if line == "" || line == "Player 1:" {
            continue;
        } else if line == "Player 2:" {
            parsing_p2 = true;
        } else {
            let number = line.parse::<u8>().expect("Should be a number");
            if !parsing_p2 {
                cards_p1.push(number);
            } else {
                cards_p2.push(number);
            }
        }
    }

    cards_p1.reverse();
    cards_p2.reverse();

    (cards_p1, cards_p2)
}

fn part1(cards_p1: &mut Vec<u8>, cards_p2: &mut Vec<u8>) {
    while !cards_p1.is_empty() && !cards_p2.is_empty() {
        println!("Player 1's deck: {:?}", cards_p1);
        println!("Player 2's deck: {:?}", cards_p2);
        let card1 = cards_p1.pop().expect("Both vecs aren't empty");
        let card2 = cards_p2.pop().expect("Both vecs aren't empty");
        println!("Player 1 plays: {:?}", card1);
        println!("Player 2 plays: {:?}", card2);

        if card1 > card2 {
            cards_p1.insert(0, card1);
            cards_p1.insert(0, card2);
        } else {
            cards_p2.insert(0, card2);
            cards_p2.insert(0, card1);
        }
    }

    println!("{:?} {:?}", cards_p1, cards_p2);

    let mut score = 0;

    if cards_p1.is_empty() {
        for i in 0..cards_p2.len() {
            score += (i+1) as u16 * cards_p2[i] as u16;
        }
    } else {
        for i in 0..cards_p1.len() {
            score += (i+1) as u16 * cards_p1[i] as u16;
        }
    }

    println!("score {}", score);
}

fn part2(lines: &Vec<String>) {
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    let (cards_p1, cards_p2) = parse(&lines);
    println!("{:?} {:?}", cards_p1, cards_p2);

    part1(&mut cards_p1.clone(), &mut cards_p2.clone());
    part2(&lines);

    Ok(())
}
