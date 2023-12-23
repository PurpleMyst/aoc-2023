use std::fmt::Display;

mod part1;
mod part2;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let part1 = part1::solve(input);

    let part2_input = input
        .replace('>', ".")
        .replace('<', ".")
        .replace('^', ".")
        .replace('v', ".");
    let part2 = part2::solve(&part2_input);

    (part1, part2)
}
