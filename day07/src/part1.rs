use std::{cmp::Ordering, fmt::Display};

const CARD_TYPES: usize = 13;
const CARDS: [u8; CARD_TYPES] = [
    b'A', b'K', b'Q', b'J', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2',
];

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
        let mut counts = [0; CARD_TYPES];
        for card in self.0 {
            counts[card as usize] += 1;
        }

        let mut most = 0;
        let mut second_most = 0;
        for &count in counts.iter() {
            if count > most {
                second_most = most;
                most = count;
            } else if count > second_most {
                second_most = count;
            }
        }

        match most {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if second_most == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if second_most == 2 {
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
        self.htype().cmp(&other.htype())
            .then_with(|| self.0.cmp(&other.0).reverse())
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
    hands
        .into_iter()
        .enumerate()
        .map(|(idx, (_, bid))| (1 + idx) * bid)
        .sum::<usize>()
}
