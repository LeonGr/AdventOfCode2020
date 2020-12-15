use std::collections::{HashMap, HashSet};

fn part1(input: &Vec<u32>) {
    let mut turn = 1;

    let mut tracker: HashMap<u32, u32> = HashMap::new();
    let mut not_first: HashSet<u32> = HashSet::new();

    for i in 0..input.len() {
        tracker.insert(input[i], turn);
        turn += 1;
    }

    let mut last: u32 = input.last().unwrap().to_owned();

    while turn <= 20 {
        match tracker.get(&last) {
            Some(spoken_turn) => {
                if not_first.contains(&last) {

                } else {
                    last = 0;
                    not_first.insert(last);
                }
            }
            None => {

            }
        }

        turn += 1;
    }
}

fn part2(input: &Vec<u32>) {
}

fn main() -> std::io::Result<()> {
    //let input = vec![18,11,9,0,5,1];
    let input = vec![0,3,6];

    part1(&input);
    part2(&input);

    Ok(())
}
