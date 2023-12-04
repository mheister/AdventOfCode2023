mod data;

use std::{cmp::max, env, fs};

use data::*;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("02/input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let games = input
        .lines()
        .map(|line| parse_game(line).and_then(|(_, game)| Ok(game)))
        .collect::<Result<Vec<Game>, _>>()
        .unwrap();
    let part1_sum = part1_sum(&games);
    println!("The sum of ids of possible games for part 1 is {part1_sum}");
    let part2_sum = part2_sum(&games);
    println!("The sum of powers of minima cube sets for part 2 is {part2_sum}");
}

fn part1_sum(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .filter_map(|game| {
            if game.reveals.iter().all(|reveal| {
                reveal.n_red <= 12 && reveal.n_green <= 13 && reveal.n_blue <= 14
            }) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

impl Game {
    fn minimal_cube_set(&self) -> CubeSet {
        self.reveals
            .iter()
            .fold(CubeSet::new(), |acc, reveal| CubeSet {
                n_red: max(acc.n_red, reveal.n_red),
                n_green: max(acc.n_green, reveal.n_green),
                n_blue: max(acc.n_blue, reveal.n_blue),
            })
    }
}

impl CubeSet {
    fn power(&self) -> usize {
        self.n_red as usize * self.n_blue as usize * self.n_green as usize
    }
}

fn part2_sum(games: &Vec<Game>) -> usize {
    games
        .iter()
        .map(|game| game.minimal_cube_set().power())
        .sum()
}
