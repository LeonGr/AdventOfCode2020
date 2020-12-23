use std::collections::VecDeque;

fn find(input: &Vec<u32>, target: u32) -> usize {
    input.iter().position(|&x| x == target).unwrap()
}

fn part1(input: &Vec<u32>) {
    let mut copy = input.clone();
    let len = input.len();
    let min = *copy.iter().min().unwrap();
    let max = *copy.iter().max().unwrap();

    let mut current_cup_index = 0;

    for _ in 0..100 {
        current_cup_index = current_cup_index % copy.len();
        let mut current_cup = copy[current_cup_index];
        //println!("current_cup {:?}", current_cup);
        //println!("order {:?}", copy);

        let mut three_cups: Vec<u32> = vec![];
        //let index = (current_cup_index + 1) % len;
        for _ in 1..=3 {
            let index = (find(&copy, current_cup) + 1) % copy.len();
            //println!("copy {:?}", copy);
            //println!("len {:?}", copy.len());
            //println!("index {}", index);
            three_cups.push(copy.remove(index));
        }

        //println!("pick up {:?}", three_cups);
        //println!("copy {:?}", copy);

        let mut destination_cup = current_cup - 1;
        loop {
            //println!("picking destination_cup {}", destination_cup);
            if three_cups.contains(&destination_cup) {
                destination_cup -= 1;
            } else if destination_cup < min {
                destination_cup = max;
            } else {
                break;
            }
        }

        //println!("destination_cup {:?}", destination_cup);

        let index_destination_cup = find(&copy, destination_cup);
        for (i, &cup) in three_cups.iter().enumerate() {
            copy.insert(index_destination_cup + i + 1, cup);
        }

        current_cup_index = find(&copy, current_cup) + 1;
        //println!("");
    }

    println!("copy {:?}", copy);

    let mut output = String::new();
    let index_1 = find(&copy, 1);
    for i in 1..copy.len() {
        output += format!("{}", copy[(index_1 + i) % copy.len()]).as_str();
    }

    println!("output {}", output);
}

fn part2(input: &Vec<u32>) {
    let mut copy = input.clone();

    let min = *copy.iter().min().unwrap();
    let max = *copy.iter().max().unwrap();

    let mut next = vec![0; input.len()+1];
    for i in 0..input.len()-1 {
        next[input[i] as usize] = input[i+1];
    }
    next[input[input.len()-1] as usize] = input[0];

    let mut current_cup = input[0];

    for i in 0..10_000_000 {
        let mut three_cups: Vec<u32> = vec![next[current_cup as usize], next[next[current_cup as usize] as usize], next[next[next[current_cup as usize] as usize] as usize]];
        next[current_cup as usize] = next[three_cups[2] as usize];

        let mut destination_cup = current_cup - 1;
        loop {
            if three_cups.contains(&destination_cup) {
                destination_cup -= 1;
            } else if destination_cup < min {
                destination_cup = max;
            } else {
                break;
            }
        }

        next[three_cups[2] as usize] = next[destination_cup as usize];
        next[destination_cup as usize] = three_cups[0];
        next[three_cups[0] as usize] = three_cups[1];
        next[three_cups[1] as usize] = three_cups[2];

        current_cup = next[current_cup as usize];
    }

    let mut copy = vec![0; input.len()];
    for i in 0..input.len() {
        current_cup = next[current_cup as usize];
        copy[i] = current_cup;
    }


    let index_1 = find(&copy, 1);
    let add_1 = copy[index_1 + 1] as u64;
    let add_2 = copy[index_1 + 2] as u64;
    let output = add_1 * add_2;

    println!("output {}", output);
}

fn main() {
    let input: Vec<u32> = "487912365".chars().map(|x| x.to_digit(10).unwrap() as u32).collect();
    //let input: Vec<u32> = "389125467".chars().map(|x| x.to_digit(10).unwrap() as u32).collect();

    //part1(&input);

    let mut part2_input = vec![];
    for i in &input {
        part2_input.push(*i);
    }
    let mut max = *input.iter().max().unwrap() + 1;
    while max <= 1_000_000 {
        part2_input.push(max);
        max += 1;
    }

    part2(&part2_input);
}
