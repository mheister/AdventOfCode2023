use std::iter::repeat;

use itertools::Itertools;

use crate::gaparangement::GapArrangement;

#[derive(Debug, PartialEq, Eq)]
pub struct SpringRow {
    pub row: String,
    pub groups: Vec<u32>,
}

impl SpringRow {
    pub fn count_possibilities(&self) -> usize {
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

    pub fn unfold(&self) -> Self {
        let row = repeat(self.row.clone()).take(5).join("?");
        let groups = repeat(self.groups.iter().cloned())
            .take(5)
            .flatten()
            .collect();
        Self { row, groups }
    }
}

#[test]
pub fn count_possibilities_1st_example_row() {
    let row = SpringRow {
        row: "???.###".to_string(),
        groups: vec![1, 1, 3],
    };
    assert_eq!(row.count_possibilities(), 1);
}

#[test]
pub fn count_possibilities_last_example_row() {
    let row = SpringRow {
        row: "?###????????".to_string(),
        groups: vec![3, 2, 1],
    };
    assert_eq!(row.count_possibilities(), 10);
}

#[test]
pub fn unfold_1st_example_row() {
    let row = SpringRow {
        row: "???.###".to_string(),
        groups: vec![1, 1, 3],
    };
    let unfolded = SpringRow {
        row: "???.###????.###????.###????.###????.###".to_string(),
        groups: vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3],
    };
    assert_eq!(row.unfold(), unfolded)
}
