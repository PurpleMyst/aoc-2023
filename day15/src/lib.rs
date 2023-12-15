use std::{fmt::Display, hash::BuildHasherDefault};

use indexmap::IndexMap;
use rustc_hash::FxHasher;

fn reindeer_hash(it: impl IntoIterator<Item = u8>) -> u8 {
    it.into_iter().fold(0, |acc, b| acc.wrapping_add(b).wrapping_mul(17))
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let init_seq = include_str!("input.txt").trim();

    let part1 = init_seq
        .split(',')
        .map(|step| reindeer_hash(step.bytes()) as u64)
        .sum::<u64>();

    let mut boxes = vec![IndexMap::with_hasher(BuildHasherDefault::<FxHasher>::default()); 256];
    for step in init_seq.split(',') {
        let splat = step.find(&['=', '-'][..]).unwrap();
        let (label, operation) = step.split_at(splat);
        let box_idx = reindeer_hash(label.bytes()) as usize;
        match operation.as_bytes()[0] {
            b'=' => {
                let focal_length = operation[1..].parse::<u8>().unwrap();
                boxes[box_idx].insert(label, focal_length);
            }

            b'-' => {
                boxes[box_idx].shift_remove(label);
            }

            _ => unreachable!(),
        }
    }

    let part2 = boxes
        .into_iter()
        .zip(1..)
        .map(|(contents, weight)| {
            weight
                * contents
                    .into_iter()
                    .zip(1..)
                    .map(|((_, focal_length), slot)| u64::from(focal_length * slot))
                    .sum::<u64>()
        })
        .sum::<u64>();

    (part1, part2)
}
