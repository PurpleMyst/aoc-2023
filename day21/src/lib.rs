use hibitset::BitSet;
use std::fmt::Display;

mod part1;
mod part2;

pub fn load_input(input: &str) -> (BitSet, usize, (usize, usize)) {
    let side = input.lines().next().unwrap().len();
    let mut start_pos = None;
    let map = input
        .lines()
        .flat_map(|line| line.bytes())
        .enumerate()
        .filter_map(|(idx, b)| match b {
            b'S' => {
                start_pos = Some((idx / side, idx % side));
                None
            }
            b'#' => Some(idx as u32),
            b'.' => None,
            _ => panic!(),
        })
        .collect();
    let start_pos = start_pos.unwrap();
    (map, side, start_pos)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let (map, side, start_pos) = load_input(input);

    let part1 = part1::solve(&map, side, start_pos, part1::TOTAL_STEPS);

    let map = part2::InfiniteGrid::new(map, side as i32);
    let part2 = part2::solve(&map, start_pos, part2::TOTAL_STEPS);

    (part1, part2)
}
