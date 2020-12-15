fn run(input: &[i32], max_turn: i32) {
    let mut tracker: Vec<i32> = vec![0; max_turn as usize];

    let len = input.len() as i32;

    for i in 0..len {
        tracker[input[i as usize] as usize] = i + 1;
    }

    let mut last_spoken: usize = input[(len - 1) as usize] as usize;

    for turn in (len + 1)..=max_turn {
        let last_seen_info = tracker[last_spoken];

        if last_seen_info == 0 {
            tracker[last_spoken] = turn - 1;
            last_spoken = 0;
        } else {
            tracker[last_spoken] = turn - 1;

            last_spoken = {
                if turn - 1 == last_seen_info {
                    0
                } else {
                    (turn - 1 - last_seen_info) as usize
                }
            };
        }
    }

    println!("last_spoken {}", last_spoken);
}

fn main() -> std::io::Result<()> {
    let input = [18, 11, 9, 0, 5, 1];

    run(&input, 2020);
    run(&input, 30000000);

    Ok(())
}
