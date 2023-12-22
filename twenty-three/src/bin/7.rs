use std::collections::{BTreeMap, HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(PartialEq, Eq)]
struct HandSummary {
    hand_rank: HandRank,
    hand: [u32; 5],
    bet: u32,
}

impl Ord for HandSummary {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_rank.cmp(&other.hand_rank) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        for i in 0..5 {
            match self.hand[i].cmp(&other.hand[i]) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            }
        }
        unreachable!()
    }
}

impl PartialOrd for HandSummary {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let card_rank: String = "AKQJT98765432".chars().rev().collect();

    let mut hand_summaries: Vec<_> = std::fs::read_to_string("7.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let hand = it.next().unwrap();
            let bet = it.next().unwrap().parse::<u32>().unwrap();

            let number_hand = to_number_hand(&hand, &card_rank);

            let cards_in_hand_by_occ = to_sorted_hand(number_hand);

            let hand_rank = match cards_in_hand_by_occ.last().unwrap().1 {
                5 => HandRank::FiveOfAKind,
                4 => HandRank::FourOfAKind,
                3 => match cards_in_hand_by_occ[cards_in_hand_by_occ.len() - 2].1 {
                    2 => HandRank::FullHouse,
                    1 => HandRank::ThreeOfAKind,
                    _ => unreachable!(),
                },
                2 => match cards_in_hand_by_occ[cards_in_hand_by_occ.len() - 2].1 {
                    2 => HandRank::TwoPair,
                    1 => HandRank::OnePair,
                    _ => unreachable!(),
                },
                _ => HandRank::HighCard,
            };

            HandSummary {
                hand_rank,
                hand: number_hand,
                bet,
            }
        })
        .collect();
    hand_summaries.sort();

    let res: u32 = hand_summaries
        .into_iter()
        .enumerate()
        .map(|(i, s)| (i + 1, s))
        .map(|(i, s)| i as u32 * s.bet)
        .sum();
    dbg!(res);
}

fn to_number_hand(s: &str, card_rank: &str) -> [u32; 5] {
    let mut a = [0; 5];
    s.chars()
        .map(|c| card_rank.find(c).unwrap() as u32)
        .enumerate()
        .for_each(|(i, n)| a[i] = n);
    a
}

// numbers ranked by most to least
// card ranking, then count
fn to_sorted_hand(s: [u32; 5]) -> Vec<(u32, u32)> {
    let mut m: HashMap<u32, u32> = HashMap::new();
    s.into_iter().for_each(|c| *m.entry(c).or_insert(0) += 1);

    let mut v: Vec<_> = m.into_iter().map(|(a, b)| (a, b)).collect();
    v.sort_by(|a, b| a.1.cmp(&b.1));
    v
}
