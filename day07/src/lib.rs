use std::fmt::Display;

mod part1;
mod part2;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    (
        part1::solve(include_str!("input.txt")),
        part2::solve(include_str!("input.txt")),
    )
}
