use std::fmt::Display;

struct Card {
    id: usize,
    matching_numbers: usize,
    amount_had: usize,
    amount_to_process: usize,
}

fn parse_numbers(s: &'static str) -> impl Iterator<Item = u64> + '_ {
    s.trim().split_ascii_whitespace().map(|s| s.parse().unwrap())
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let mut cards = input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let (winners, candidates) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let winners = parse_numbers(winners).collect::<Vec<_>>();
            let matching_numbers = parse_numbers(candidates)
                .filter(|n| winners.contains(&n))
                .count();
            Card {
                id: id + 1,
                matching_numbers,
                amount_had: 1,
                amount_to_process: 1,
            }
        })
        .collect::<Vec<_>>();

    let part1 = cards
        .iter()
        .map(|&Card { matching_numbers, .. }| {
            if matching_numbers == 0 {
                0
            } else {
                1 << (matching_numbers - 1)
            }
        })
        .sum::<usize>();

    for idx in 0..cards.len() {
        let Card {
            amount_had: _,
            amount_to_process,
            matching_numbers,
            id,
        } = &mut cards[idx];
        let id = *id;
        let amount_to_add = *amount_to_process;
        *amount_to_process = 0;
        for won_id in id + 1..=id + *matching_numbers {
            cards[won_id - 1].amount_had += amount_to_add;
            cards[won_id - 1].amount_to_process += amount_to_add;
        }
    }

    let part2 = cards.into_iter().map(|card| card.amount_had).sum::<usize>();

    (part1, part2)
}
