mod data;

use std::{collections::HashSet, env, fs};

use data::*;

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("04/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let cards = input
        .lines()
        .map(|line| parse_card(line).and_then(|(_, card)| Ok(card)))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let part1_score = calc_score_part1(&cards);
    println!("The score for part 1 is {part1_score}");
    let part2_nof_cards = calc_total_nof_cards_part2(&cards);
    println!("The total number of cards for part 2 is {part2_nof_cards}");
}

impl Scratchcard {
    fn calc_match_count(&self) -> usize {
        let winning: HashSet<u32> = HashSet::from_iter(self.winning.iter().cloned());
        // if our numbers don't repreat:
        let have: HashSet<u32> = HashSet::from_iter(self.have.iter().cloned());
        let match_count = winning.intersection(&have).count();
        // let match_count = card.have.iter().filter(|h| winning.contains(h)).count();
        match_count
    }
}

fn calc_score_part1(cards: &Vec<Scratchcard>) -> usize {
    cards
        .iter()
        .map(|card| {
            let match_count = card.calc_match_count();
            if match_count > 0 {
                2_usize.pow((match_count - 1).try_into().unwrap())
            } else {
                0
            }
        })
        .sum()
}

fn calc_total_nof_cards_part2(cards: &Vec<Scratchcard>) -> usize {
    let mut counts = vec![1; cards.len()];
    for i in 0..cards.len() {
        let match_count = cards[i].calc_match_count();
        for card_won_idx in i + 1..=i + match_count {
            counts[card_won_idx] += counts[i];
        }
    }
    counts.iter().sum()
}
