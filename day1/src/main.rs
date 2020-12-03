use std::io;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;

fn read_lines_to_int() -> io::Result<Vec<i32>> {
    let input_file = fs::File::open("input")?;
    let file_reader = BufReader::new(input_file);

    Ok(file_reader
       .lines()
       .filter_map(io::Result::ok)
       .map(|string| string.parse::<i32>().unwrap())
       .collect())
}

fn main() -> io::Result<()> {
    let mut input = read_lines_to_int()?;

    for i in (0..input.len()).rev() {
        for j in 0..input.len() {
            for k in 0..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    println!("{}*{}*{}={}", input[i], input[j], input[k], input[i] * input[j] * input[k]);
                }
            }
        }
    }

    Ok(())
}
