use std::{env, fs};

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("07/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    let mut hands = input
        .lines()
        .map(|l| l.try_into().unwrap())
        .collect::<Vec<Hand>>();
    hands.sort();
    let ranks = (1..=hands.len()).rev();
    let p1_winnings: usize = hands
        .iter()
        .zip(ranks.clone())
        .map(|(hand, rank)| hand.bid as usize * rank)
        .sum();
    println!("Winnings in part 1: {p1_winnings}");
    let mut jhands = input
        .lines()
        .map(|l| l.try_into().unwrap())
        .collect::<Vec<JHand>>();
    jhands.sort();
    let p2_winnings: usize = jhands
        .iter()
        .zip(ranks)
        .map(|(hand, rank)| hand.bid as usize * rank)
        .sum();
    println!("Winnings in part 2: {p2_winnings}");
}

#[derive(Clone, PartialEq, Eq, Ord)]
struct Hand {
    cards: [char; 5],
    bid: u32,
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {}",
            &self.cards.iter().collect::<String>(),
            self.bid
        ))
    }
}

fn cards_from_str(cards: &str) -> Result<[char; 5], String> {
    if cards.len() != 5 {
        return Err("hand str must be 5 characters long".to_string());
    }
    if let Some(c) = cards.chars().find(|c| {
        ![
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ]
        .contains(&c)
    }) {
        return Err(format!("invalid card character '{c}'"));
    }
    let mut chars = cards.chars();
    let cards = [
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
    ];
    Ok(cards)
}

impl TryFrom<&str> for Hand {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (cards, bid) = value.split_once(' ').ok_or(format!(
            "Error parsing hand, expecting two words for hand and bid, got '{value}'"
        ))?;
        let cards = cards_from_str(cards)?;
        Ok(Hand {
            cards,
            bid: bid
                .parse()
                .map_err(|_| format!("Could not parse bid amount in '{value}'"))?,
        })
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn histogram(&self) -> [u8; 13] {
        let mut counts = [0u8; 13];
        for x in self.cards {
            let bin = [
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ]
            .iter()
            .position(|&b| b == x)
            .unwrap();
            counts[bin as usize] += 1;
        }
        counts
    }

    fn determine_type(&self) -> Type {
        let hist = self.histogram();
        if hist.contains(&5) {
            return Type::FiveOfAKind;
        }
        if hist.contains(&4) {
            return Type::FourOfAKind;
        }
        if hist.contains(&3) && hist.contains(&2) {
            return Type::FullHouse;
        }
        if hist.contains(&3) {
            return Type::ThreeOfAKind;
        }
        if hist.iter().filter(|x| **x == 2).count() == 2 {
            return Type::TwoPair;
        }
        if hist.contains(&2) {
            return Type::OnePair;
        }
        Type::HighCard
    }
}

// the lower the stronger
fn strength(card: char) -> u8 {
    [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ]
    .iter()
    .position(|&b| b == card)
    .unwrap() as u8
}

fn compare_hands_of_same_type(h1: [char; 5], h2: [char; 5]) -> Option<std::cmp::Ordering> {
    h1.iter()
        .zip(h2.iter())
        .find(|(x, y)| x != y)
        .map(|(x, y)| strength(*x).partial_cmp(&strength(*y)))
        .unwrap_or(Some(core::cmp::Ordering::Equal))
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.determine_type().partial_cmp(&other.determine_type()) {
            Some(core::cmp::Ordering::Equal) => {
                compare_hands_of_same_type(self.cards, other.cards)
            }
            ord => return ord,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Ord)]
struct JHand {
    cards: [char; 5],
    bid: u32,
}

impl std::fmt::Debug for JHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {}\t{:?}",
            &self.cards.iter().collect::<String>(),
            self.bid,
            self.determine_type()
        ))
    }
}

impl TryFrom<&str> for JHand {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (cards, bid) = value.split_once(' ').ok_or(format!(
            "Error parsing hand, expecting two words for hand and bid, got '{value}'"
        ))?;
        let cards = cards_from_str(cards)?;
        Ok(JHand {
            cards,
            bid: bid
                .parse()
                .map_err(|_| format!("Could not parse bid amount in '{value}'"))?,
        })
    }
}

impl JHand {
    fn histogram(&self) -> [u8; 13] {
        let mut counts = [0u8; 13];
        for x in self.cards {
            let bin = [
                'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
            ]
            .iter()
            .position(|&b| b == x)
            .unwrap();
            counts[bin as usize] += 1;
        }
        counts
    }

    fn determine_type(&self) -> Type {
        let hist = self.histogram();

        let n_equal_with_j = |n, hist: [u8; 13]| -> Option<[u8; 13]> {
            let jokers = hist[12];
            if let Some((pos, jokers_used)) = (0..=jokers).find_map(|jokers_used| {
                hist[0..12]
                    .iter()
                    .position(|&x| x == (n - jokers_used))
                    .map(|pos| (pos, jokers_used))
            }) {
                let mut new_hist = hist.clone();
                new_hist[pos] -= n - jokers_used;
                new_hist[12] -= jokers_used;
                return Some(new_hist);
            }
            if hist[12] >= n {
                let mut new_hist = hist.clone();
                new_hist[12] -= n;
                return Some(new_hist);
            }
            None
        };

        if n_equal_with_j(5, hist).is_some() {
            return Type::FiveOfAKind;
        }
        if n_equal_with_j(4, hist).is_some() {
            return Type::FourOfAKind;
        }
        if let Some(hist) = n_equal_with_j(3, hist) {
            if n_equal_with_j(2, hist).is_some() {
                return Type::FullHouse;
            }
        }
        if n_equal_with_j(3, hist).is_some() {
            return Type::ThreeOfAKind;
        }
        if n_equal_with_j(2, hist)
            .and_then(|hist| n_equal_with_j(2, hist))
            .is_some()
        {
            return Type::TwoPair;
        }
        if n_equal_with_j(2, hist).is_some() {
            return Type::OnePair;
        }
        Type::HighCard
    }
}

// the lower the stronger
fn strength_j(card: char) -> u8 {
    [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ]
    .iter()
    .position(|&b| b == card)
    .unwrap() as u8
}

fn compare_hands_of_same_type_j(
    h1: [char; 5],
    h2: [char; 5],
) -> Option<std::cmp::Ordering> {
    h1.iter()
        .zip(h2.iter())
        .find(|(x, y)| x != y)
        .map(|(x, y)| strength_j(*x).partial_cmp(&strength_j(*y)))
        .unwrap_or(Some(core::cmp::Ordering::Equal))
}

impl PartialOrd for JHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.determine_type().partial_cmp(&other.determine_type()) {
            Some(core::cmp::Ordering::Equal) => {
                compare_hands_of_same_type_j(self.cards, other.cards)
            }
            ord => return ord,
        }
    }
}
