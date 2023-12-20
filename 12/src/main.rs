mod gaparangement;

use std::{env, fs, iter::repeat};

use crate::gaparangement::GapArrangement;

#[derive(Debug, PartialEq, Eq)]
struct SpringRow {
    row: String,
    groups: Vec<u32>,
}

impl SpringRow {
    fn count_possibilities(&self) -> usize {
        GapArrangement {
            n_gaps: (self.groups.len() + 1).try_into().unwrap(),
            total_gap_len: (self.row.len()
                - self.groups.iter().cloned().sum::<u32>() as usize)
                .try_into()
                .unwrap(),
        }
        .into_iter()
        .map(|gaps| {
            let mut row = repeat('.')
                .take(*gaps.first().unwrap_or(&0) as usize)
                .collect::<String>();
            for (&grp, &gap) in self.groups.iter().zip(gaps.iter().skip(1)) {
                row.extend(repeat('#').take(grp as usize));
                row.extend(repeat('.').take(gap as usize));
            }
            row
        })
        .filter(|row_canditate| {
            self.row
                .chars()
                .zip(row_canditate.chars())
                .all(|(r, c)| match (r, c) {
                    ('?', _) => true,
                    (r, c) => r == c,
                })
        })
        .count()
    }
}

#[test]
fn count_possibilities_1st_example_row() {
    let row = SpringRow {
        row: "???.###".to_string(),
        groups: vec![1, 1, 3],
    };
    assert_eq!(row.count_possibilities(), 1);
}

#[test]
fn count_possibilities_last_example_row() {
    let row = SpringRow {
        row: "?###????????".to_string(),
        groups: vec![3, 2, 1],
    };
    assert_eq!(row.count_possibilities(), 10);
}

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("12/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let sum_of_possibilities: usize = input
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
            .count_possibilities()
        })
        .sum();
    dbg!(sum_of_possibilities);
}
