use std::{env, fs};

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("01/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let sum: u32 = input
        .lines()
        .map(|line| {
            let digit1 = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .and_then(|c| c.to_digit(10))
                .unwrap();
            let digit2 = line
                .chars()
                .rfind(|c| c.is_ascii_digit())
                .and_then(|c| c.to_digit(10))
                .unwrap();
            digit1 * 10 + digit2
        })
        .sum();
    println!("The sum of calibration values is {sum}");
}
