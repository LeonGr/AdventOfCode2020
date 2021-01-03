fn transform(value: u64, subject: u64, modulo: u64) -> u64 {
    value * subject % modulo
}

fn part1() {
    let card_pub: u64 = 15335876;
    let door_pub: u64 = 15086442;
    let modulo = 20201227;

    let mut value;
    let mut subject;

    value = 1;
    subject = 7;

    let mut card_loop_size = 0;
    while value != card_pub {
        value = transform(value, subject, modulo);
        card_loop_size += 1;
    }

    println!("card_pub card_loop_size {}", card_loop_size);

    value = 1;
    subject = 7;

    let mut door_loop_size = 0;
    while value != door_pub {
        value = transform(value, subject, modulo);
        door_loop_size += 1;
    }

    println!("door_pub door_loop_size {}", door_loop_size);

    value = 1;
    subject = door_pub;
    for _ in 0..card_loop_size {
        value = transform(value, subject, modulo);
    }

    let enc_key = value;
    println!("encryption key {}", enc_key);
}

fn part2() {
}

fn main() -> std::io::Result<()> {
    part1();
    part2();

    Ok(())
}
