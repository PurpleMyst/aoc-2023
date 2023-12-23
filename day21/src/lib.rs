use std::fmt::Display;

mod part1;
mod part2;

pub fn load_input(input: &str) -> (grid::Grid<bool>, (usize, usize)) {
    let columns = input.lines().next().unwrap().len();
    let mut start_pos = None;
    let map = grid::Grid::from_vec(
        input
            .lines()
            .flat_map(|line| line.bytes())
            .enumerate()
            .map(|(idx, b)| match b {
                b'S' => {
                    start_pos = Some((idx / columns, idx % columns));
                    b'.'
                }
                b'#' | b'.' => b,
                _ => panic!(),
            })
            .map(|b| b == b'#')
            .collect(),
        columns,
    );
    let start_pos = start_pos.unwrap();
    (map, start_pos)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let (map, start_pos) = load_input(input);

    let part1 = part1::solve(&map, start_pos, part1::TOTAL_STEPS);

    let map = part2::InfiniteGrid::new(map);
    let part2 = part2::solve(&map, start_pos, part2::TOTAL_STEPS);

    (part1, part2)
}
