use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Seen {
    first: u32,
    last: u32,
}

fn run(input: &[u32], max_turn: u32) {
    let mut turn = 1;

    let mut tracker: HashMap<u32, Seen> = HashMap::new();

    for i in 0..input.len() {
        tracker.insert(input[i], Seen { first: turn, last: turn });
        turn += 1;
    }

    let mut last_spoken = input.last().unwrap().to_owned();

    while turn <= max_turn {
        match tracker.get(&last_spoken) {
            Some(last_seen_info) => {
                let (first, last) = (last_seen_info.first, last_seen_info.last);

                tracker.insert(last_spoken, Seen {first, last: turn - 1});

                if turn - 1 == last {
                    last_spoken = 0;
                } else {
                    last_spoken = turn - 1 - last;
                }
            }
            None => {
                tracker.insert(last_spoken, Seen { first: turn - 1, last: turn - 1 });
                last_spoken = 0;
            }
        }

        turn += 1;
    }

    println!("last_spoken {}", last_spoken);
}

fn main() -> std::io::Result<()> {
    let input = [18,11,9,0,5,1];

    run(&input, 2020u32);
    run(&input, 30000000u32);

    Ok(())
}
