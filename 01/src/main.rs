use std::{collections::HashMap, env, fs};

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("01/input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let sum_p1 = part1_calibration_sum(&input);
    println!("The sum of calibration values is {sum_p1}");
    let sum_p2 = part2_calibration_sum(&input);
    println!("The sum of calibration values considering spelled out digits is {sum_p2}");
}

fn part1_calibration_sum(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digit1 = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .and_then(|c| c.to_digit(10))
                .unwrap_or(0);
            let digit2 = line
                .chars()
                .rfind(|c| c.is_ascii_digit())
                .and_then(|c| c.to_digit(10))
                .unwrap_or(0);
            digit1 * 10 + digit2
        })
        .sum()
}

fn part2_calibration_sum(input: &str) -> u32 {
    let digits = HashMap::<&str, u8>::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    input
        .lines()
        .map(|line| {
            let digit1 = {
                digits
                    .keys()
                    .filter_map(|digit_str| {
                        str::find(line, digit_str).map(|pos| (pos, digits[digit_str]))
                    })
                    .min_by(|x, y| x.0.cmp(&y.0))
                    .unwrap()
                    .1
            };
            let digit2 = {
                digits
                    .keys()
                    .filter_map(|digit_str| {
                        str::rfind(line, digit_str).map(|pos| (pos, digits[digit_str]))
                    })
                    .max_by(|x, y| x.0.cmp(&y.0))
                    .unwrap()
                    .1
            };
            digit1 as u32 * 10 + digit2 as u32
        })
        .sum()
}
