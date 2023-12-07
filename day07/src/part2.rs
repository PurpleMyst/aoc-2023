use std::{cmp::Ordering, fmt::Display};

const CARD_TYPES: usize = 13;
const CARDS: [u8; CARD_TYPES] = [
    b'A', b'K', b'Q', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2', b'J',
];
const JOLLY: u8 = CARD_TYPES as u8 - 1;

#[derive(Eq, PartialEq, Clone, Copy)]
struct Hand([u8; 5]);

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self.0.iter().map(|&c| CARDS[c as usize] as char).collect::<String>();
        f.pad(&cards)
    }
}

impl Hand {
    fn new(s: &str) -> Self {
        debug_assert_eq!(s.len(), 5);
        let mut cards = [0; 5];
        cards
            .iter_mut()
            .zip(s.bytes())
            .for_each(|(c, b)| *c = CARDS.iter().position(|&x| x == b).unwrap() as u8);
        Self(cards)
    }

    fn htype(&self) -> HandType {
        let mut counts = [0usize; CARD_TYPES - 1];
        let mut jollies = 0usize;

        for card in self.0 {
            if card == JOLLY {
                jollies += 1;
            } else {
                counts[card as usize] += 1;
            }
        }

        counts.sort_unstable();
        counts.reverse();
        match counts[0] + jollies {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                let remaining_jollies = jollies - (3 - counts[0]);
                if counts[1..].iter().find(|&&c| c == 2 - remaining_jollies).is_some() {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                let remaining_jollies = jollies - (2 - counts[0]);
                if counts[1..].iter().find(|&&c| c == 2 - remaining_jollies).is_some() {
                    HandType::TwoPair
                } else {
                    HandType::Pair
                }
            }
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(self.htype().cmp(&other.htype()), |acc, (&a, &b)| {
                acc.then(a.cmp(&b).reverse())
            })
    }
}

pub(super) fn solve(input: &str) -> impl Display {
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = Hand::new(hand);
            let bid = bid.parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by_key(|&(hand, _)| hand);
    let answer = hands
        .into_iter()
        .enumerate()
        .map(|(idx, (_, bid))| (1 + idx) * bid)
        .sum::<usize>();
    if answer > 249749038 {
        format!("\x1b[32m{answer}\x1b[0m")
    } else {
        format!("\x1b[31m{answer}\x1b[0m")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jollies() {
        use HandType::*;
        let cases = [("82JT2", ThreeOfAKind), ("Q8833", TwoPair)];
        for (hand, htype) in cases {
            let actual = Hand::new(hand).htype();
            assert_eq!(actual, htype, "{hand}'s type was {actual:?}, expected {htype:?}");
        }
    }
}
