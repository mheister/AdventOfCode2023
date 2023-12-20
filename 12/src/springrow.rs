use std::{collections::HashMap, iter::repeat};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct SpringRow {
    pub row: String,
    pub groups: Vec<u32>,
}

// 2 stage map so we can lookup via references
type MemMap = HashMap<String, HashMap<Vec<u32>, usize>>;

impl SpringRow {
    pub fn count_possibilities(&self) -> usize {
        Self::count_possibilities_fast_internal(
            &self.row,
            &self.groups,
            &mut HashMap::new(),
        )
    }

    fn count_possibilities_fast_internal(
        row: &str,
        groups: &[u32],
        mem: &mut MemMap,
    ) -> usize {
        if groups.is_empty() {
            return 1;
        }
        if row.is_empty() {
            return 0;
        }
        if let Some(&ref counts_for_this_string) = mem.get(row) {
            if let Some(&cnt) = counts_for_this_string.get(groups) {
                return cnt;
            }
        }
        let grp0 = groups[0] as usize;
        let grps = &groups[1..];
        let pos_max: usize = row.len().saturating_sub(
            grp0 + (grps.len().saturating_sub(1)) + grps.iter().sum::<u32>() as usize,
        );
        // group begins latest at first known broken spring
        let pos_max = std::cmp::min(
            pos_max,
            row.chars().position(|c| c == '#').unwrap_or(pos_max),
        );
        let mut sum = 0;
        for i in 0..=pos_max {
            if i > row.len() {
                dbg!(row, i, groups);
            }
            if i > 0 && row.chars().nth(i - 1).unwrap() == '#' {
                // need a gap to begin a group
                continue;
            }
            if row.chars().nth(i + grp0).unwrap_or('.') == '#' {
                // need a gap towards the next group
                continue;
            }
            if row.chars().skip(i).take(grp0).any(|c| c == '.') {
                continue;
            }
            if row.len() < i + grp0 {
                continue;
            }
            if row.len() <= i + grp0 + 1 {
                if grps.is_empty() {
                    sum += 1;
                }
                continue;
            }
            sum +=
                Self::count_possibilities_fast_internal(&row[i + grp0 + 1..], grps, mem);
        }
        mem.entry(row.to_string()).or_default().insert(groups.to_vec(), sum);
        sum
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn count_possibilities_bug1() {
        let row = SpringRow {
            row: "????###????????".to_string(),
            groups: vec![3, 2, 1],
        };
        assert_eq!(row.count_possibilities(), 10);
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
    pub fn count_possibilities_4th_example_row() {
        let row = SpringRow {
            row: "????.#...#...".to_string(),
            groups: vec![4, 1, 1],
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

    #[test]
    pub fn count_possibilities_unfolded_1st_example_row() {
        let row = SpringRow {
            row: "???.###".to_string(),
            groups: vec![1, 1, 3],
        }
        .unfold();
        assert_eq!(row.count_possibilities(), 1);
    }

    #[test]
    pub fn count_possibilities_unfolded_2nd_example_row() {
        let row = SpringRow {
            row: ".??..??...?##.".to_string(),
            groups: vec![1, 1, 3],
        }
        .unfold();
        assert_eq!(row.count_possibilities(), 16384);
    }

    #[test]
    pub fn count_possibilities_unfolded_3rd_example_row() {
        let row = SpringRow {
            row: "?#?#?#?#?#?#?#?".to_string(),
            groups: vec![1, 3, 1, 6],
        }
        .unfold();
        assert_eq!(row.count_possibilities(), 1);
    }

    #[test]
    pub fn count_possibilities_unfolded_unfolded_4th_example_row() {
        let row = SpringRow {
            row: "????.#...#...".to_string(),
            groups: vec![4, 1, 1],
        }
        .unfold();
        assert_eq!(row.count_possibilities(), 16);
    }

    #[test]
    pub fn count_possibilities_unfolded_unfolded_5th_example_row() {
        let row = SpringRow {
            row: "????.######..#####.".to_string(),
            groups: vec![1, 6, 5],
        }
        .unfold();
        assert_eq!(row.count_possibilities(), 2500);
    }

    #[test]
    pub fn count_possibilities_unfolded_last_example_row() {
        let row = SpringRow {
            row: "?###????????".to_string(),
            groups: vec![3, 2, 1],
        }
        .unfold();
        assert_eq!(row.count_possibilities(), 506250);
    }
}
