use std::collections::HashMap;

fn run(input: &[u32], max_turn: u32) {
    let mut turn = 1;

    let mut tracker: HashMap<u32, u32> = HashMap::new();

    for i in 0..input.len() {
        tracker.insert(input[i], turn);
        turn += 1;
    }

    let mut last_spoken = input.last().unwrap().to_owned();

    while turn <= max_turn {
        match tracker.get(&last_spoken) {
            Some(last_seen_info) => {
                let last = last_seen_info.to_owned();

                tracker.insert(last_spoken, turn - 1);

                last_spoken = {
                    if turn - 1 == last {
                        0
                    } else {
                        turn - 1 - last
                    }
                }
            }
            None => {
                tracker.insert(last_spoken, turn - 1);
                last_spoken = 0;
            }
        }

        turn += 1;
    }

    println!("last_spoken {}", last_spoken);
}

fn main() -> std::io::Result<()> {
    let input = [18, 11, 9, 0, 5, 1];

    run(&input, 2020u32);
    run(&input, 30000000u32);

    Ok(())
}
