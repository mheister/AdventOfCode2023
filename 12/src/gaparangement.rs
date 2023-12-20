use std::mem::replace;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GapArrangement {
    pub n_gaps: u32,
    pub total_gap_len: u32,
}

impl IntoIterator for GapArrangement {
    type Item = Vec<u32>;

    type IntoIter = GapArrangementIter;

    fn into_iter(self) -> Self::IntoIter {
        GapArrangementIter::new(self.n_gaps, self.total_gap_len)
    }
}

// iterate over potential sizes of gaps given the total length
#[derive(Debug, PartialEq, Eq)]
pub struct GapArrangementIter {
    gaps: Vec<u32>,
}

impl GapArrangementIter {
    fn new(n_gaps: u32, total_gap_len: u32) -> Self {
        assert!(n_gaps >= 2);
        let mut gaps = vec![1; n_gaps as usize];
        *gaps.first_mut().unwrap() = total_gap_len + 2 - n_gaps;
        *gaps.last_mut().unwrap() = 0;
        Self { gaps }
    }
}

impl Iterator for GapArrangementIter {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.gaps.is_empty() {
            return None;
        }
        let res = Some(self.gaps.clone());
        // if self.gaps.iter().rev().skip(1).all(|&c| c == 1) {
        //     self.gaps.clear();
        // }
        if *self.gaps.first().unwrap_or(&0) > 0 {
            assert!(self.gaps.len() >= 2);
            self.gaps[0] -= 1;
            self.gaps[1] += 1;
            return res;
        }
        if let Some(shift_pos) = self.gaps.iter().rev().skip(1).rev().position(|&g| g > 1)
        {
            // take 1 from shift pos to the next position and all but 1 to first position
            self.gaps[shift_pos + 1] += 1;
            let remaining = replace(&mut self.gaps[shift_pos], 1) - 2;
            self.gaps[0] += remaining;
            return res;
        }
        self.gaps.clear();
        return res;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn gaparangement_yields_arrangements_3_gaps_5_len() {
        let mut gapit = GapArrangementIter::new(3, 5);
        assert_eq!(gapit.next(), Some(vec![4, 1, 0]));
        assert_eq!(gapit.next(), Some(vec![3, 2, 0]));
        assert_eq!(gapit.next(), Some(vec![2, 3, 0]));
        assert_eq!(gapit.next(), Some(vec![1, 4, 0]));
        assert_eq!(gapit.next(), Some(vec![0, 5, 0]));
        assert_eq!(gapit.next(), Some(vec![3, 1, 1]));
        assert_eq!(gapit.next(), Some(vec![2, 2, 1]));
        assert_eq!(gapit.next(), Some(vec![1, 3, 1]));
        assert_eq!(gapit.next(), Some(vec![0, 4, 1]));
        assert_eq!(gapit.next(), Some(vec![2, 1, 2]));
        assert_eq!(gapit.next(), Some(vec![1, 2, 2]));
        assert_eq!(gapit.next(), Some(vec![0, 3, 2]));
        assert_eq!(gapit.next(), Some(vec![1, 1, 3]));
        assert_eq!(gapit.next(), Some(vec![0, 2, 3]));
        assert_eq!(gapit.next(), Some(vec![0, 1, 4]));
        assert_eq!(gapit.next(), None);
    }
}
