use std::io::BufRead;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}
fn part1(lines: &Vec<String>) {
    let departure = lines[0].parse::<u32>().unwrap();
    let mut times_ids = lines[1]
        .split(",")
        .filter(|x| x != &"x")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    times_ids
        .sort_by(|a, b| {
            (departure / a * a).partial_cmp(&(departure / b * b)).unwrap()
        });

    let bus_id_time = times_ids[0];
    let earliest = departure / bus_id_time * bus_id_time + bus_id_time;
    println!("{:?}", (earliest - departure) * bus_id_time);
}

fn modular_inverse(a: i64, modulo: i64) -> i64 {
    let mut mn = (modulo, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += modulo;
    }

    xy.0
}

fn part2(lines: &Vec<String>) {
    let times_ids = lines[1]
            .split(",")
            .collect::<Vec<&str>>();

    println!("{:?}", times_ids);

    let non_x: Vec<(i64, i64)> = times_ids
        .iter()
        .enumerate()
        .map(|(i, &x)| (i, x))
        .filter(|(i, x)| x != &"x")
        .map(|(i, x)| (i as i64, x.parse::<i64>().unwrap()))
        .collect();

    println!("non_x {:?}", non_x);

    let n = non_x
        .iter()
        .fold(1, |acc, (i, x)| acc * x);

    println!("n {:?}", n);

    let m: Vec<i64> = non_x
        .iter()
        .map(|(i, x)| n / x)
        .collect();

    println!("m {:?}", m);

    let a: Vec<i64> = non_x
        .iter()
        .map(|(i, x)| (x - i) % x)
        .collect();

    println!("a {:?}", a);

    let c: Vec<i64> = m
        .iter()
        .enumerate()
        .map(|(i, &x)| x * modular_inverse(x as i64, non_x[i].1 as i64))
        .collect();

    println!("c {:?}", c);

    let mut t = 0;
    for i in 0..non_x.len() {
        t += a[i] * c[i];
    }

    println!("t {}", t % n);
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    part1(&lines);
    part2(&lines);

    //println!("{}", modular_inverse(5, 13));

    Ok(())
}
