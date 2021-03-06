use std::{ io::BufRead, collections::{ HashSet, HashMap } };
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn parse(lines: &Vec<String>) -> (Vec<u16>, Vec<u16>) {
    let mut parsing_p2 = false;

    let mut cards_p1: Vec<u16> = vec![];
    let mut cards_p2: Vec<u16> = vec![];

    for line in lines {
        if line == "" || line == "Player 1:" {
            continue;
        } else if line == "Player 2:" {
            parsing_p2 = true;
        } else {
            let number = line.parse::<u16>().expect("Should be a number");
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

fn calculate_score(cards: &Vec<u16>) -> u16 {
    cards
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i+1) as u16 * x)
}

#[derive(Clone, Debug)]
enum Winner {
    Player1,
    Player2,
}

#[derive(Clone)]
struct Outcome {
    winner: Winner,
    cards: Vec<u16>,
}

fn play_combat(cards_p1: &mut Vec<u16>, cards_p2: &mut Vec<u16>) -> Outcome {
    while !cards_p1.is_empty() && !cards_p2.is_empty() {
        let card1 = cards_p1.pop().expect("Both vecs aren't empty");
        let card2 = cards_p2.pop().expect("Both vecs aren't empty");

        if card1 > card2 {
            cards_p1.insert(0, card1);
            cards_p1.insert(0, card2);
        } else {
            cards_p2.insert(0, card2);
            cards_p2.insert(0, card1);
        }
    }

    if cards_p1.is_empty() {
        Outcome { winner: Winner::Player2, cards: cards_p2.clone() }
    } else {
        Outcome { winner: Winner::Player1, cards: cards_p1.clone() }
    }
}

fn part1(cards_p1: &mut Vec<u16>, cards_p2: &mut Vec<u16>) {
    let outcome = play_combat(cards_p1, cards_p2);
    let winners_cards = outcome.cards;
    let score = calculate_score(&winners_cards);

    println!("{:?} won, score {}", outcome.winner, score);
}

fn hashify(cards_p1: &Vec<u16>, cards_p2: &Vec<u16>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for num in cards_p1 {
        hasher.write_u16(*num);
    }
    for num in cards_p2 {
        hasher.write_u16(*num);
    }
    hasher.finish()
}

fn play_recursive_combat(cards_p1: &mut Vec<u16>, cards_p2: &mut Vec<u16>, seen_games: &mut HashMap<u64, Outcome>) -> Outcome {
    let mut previous_rounds: HashSet<u64> = HashSet::new();

    let initial_state = hashify(cards_p1, cards_p2);
    match seen_games.get(&initial_state) {
        Some(result) => {
            return result.clone()
        },
        None => (),
    };

    while !cards_p1.is_empty() && !cards_p2.is_empty() {
        let state_string = hashify(cards_p1, cards_p2);

        match seen_games.get(&state_string) {
            Some(result) => {
                return result.clone()
            },
            None => (),
        };

        if previous_rounds.contains(&state_string) {
            return Outcome { winner: Winner::Player1, cards: cards_p1.clone() };
        }

        previous_rounds.insert(state_string.clone());

        let card1 = cards_p1.pop().expect("Both vecs cannot be empty");
        let card2 = cards_p2.pop().expect("Both vecs cannot be empty");

        let winner = {
            let len1 = cards_p1.len();
            let len2 = cards_p2.len();
            if len1 >= card1 as usize && len2 >= card2 as usize {
                let subgame_cards_p1: Vec<u16> = cards_p1[len1 - card1 as usize..].to_vec();
                let subgame_cards_p2: Vec<u16> = cards_p2[len2 - card2 as usize..].to_vec();
                let outcome = play_recursive_combat(&mut subgame_cards_p1.clone(), &mut subgame_cards_p2.clone(), seen_games);
                seen_games.insert(state_string, outcome.clone());
                outcome.winner
            } else {
                if card1 > card2 {
                    Winner::Player1
                } else {
                    Winner::Player2
                }
            }
        };

        match winner {
            Winner::Player1 => {
                cards_p1.insert(0, card1);
                cards_p1.insert(0, card2);
            }
            Winner::Player2 => {
                cards_p2.insert(0, card2);
                cards_p2.insert(0, card1);
            }
        }
    }

    let outcome = {
        if cards_p1.is_empty() {
            Outcome { winner: Winner::Player2, cards: cards_p2.clone() }
        } else {
            Outcome { winner: Winner::Player1, cards: cards_p1.clone() }
        }
    };
    seen_games.insert(initial_state, outcome.clone());
    outcome
}

fn part2(cards_p1: &mut Vec<u16>, cards_p2: &mut Vec<u16>) {
    let mut seen_games: HashMap<u64, Outcome> = HashMap::new();
    let outcome = play_recursive_combat(cards_p1, cards_p2, &mut seen_games);
    let winners_cards = outcome.cards;
    let score = calculate_score(&winners_cards);

    println!("{:?} won, score {}", outcome.winner, score);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    let (cards_p1, cards_p2) = parse(&lines);

    part1(&mut cards_p1.clone(), &mut cards_p2.clone());
    part2(&mut cards_p1.clone(), &mut cards_p2.clone());

    Ok(())
}
