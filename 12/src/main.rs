mod springrow;

use std::{env, fs};

use crate::springrow::SpringRow;


fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("12/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let spring_rows = input
        .lines()
        .map(|l| {
            let (row, grp_str) = l.split_once(' ').unwrap();
            SpringRow {
                row: row.to_string(),
                groups: grp_str
                    .split(',')
                    .map(str::parse::<u32>)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap(),
            }
        })
        .collect::<Vec<_>>();
    let sum_of_possibilities: usize =
        spring_rows.iter().map(|r| r.count_possibilities()).sum();
    println!("The sum of possibilities for part 1 is {sum_of_possibilities}");
    let sum_of_possibilities_p2: usize = spring_rows
        .iter()
        .map(|r| r.unfold().count_possibilities())
        .sum();
    println!(
        "The sum of possibilities after expanding for part 2 is {sum_of_possibilities_p2}"
    );
}
