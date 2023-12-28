use std::fmt::Display;

pub mod part1;
pub mod part2;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    (part1::solve(input), part2::solve(input))
}
