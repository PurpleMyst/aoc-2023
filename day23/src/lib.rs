use std::fmt::Display;

pub mod part1;
pub mod part2;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let part1 = part1::solve(input);

    let part2_input = input.replace(['>', '<', '^', 'v'], ".");
    let part2 = part2::solve(&part2_input);

    (part1, part2)
}
