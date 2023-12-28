use std::fmt::Display;

use ::bucket_queue::{BucketQueue, LastInFirstOutQueue as _, Queue as _};
use hibitset::BitSet;
use rustc_hash::FxHashSet as HashSet;

const P1_TOTAL_STEPS: usize = 64;
const P2_TOTAL_STEPS: usize = 26_501_365;

// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
fn load_input(input: &str) -> (BitSet, usize, (usize, usize)) {
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
    let (walls, side, start_pos) = load_input(input);

        let mut q = BucketQueue::<Vec<_>>::new();
        q.push(start_pos, 0);

        let mut visited = HashSet::default();

        let mut part1 = 0;

        let mut odd_full = 0;
        let mut even_full = 0;
        let mut odd_corners = 0;
        let mut even_corners = 0;

        while let Some(distance) = q.min_priority() {
            let (x, y) = q.pop(distance).unwrap();
            if !visited.insert((x, y)) {
                continue;
            }

            if distance < P1_TOTAL_STEPS {
                part1 += 1;
            }

            if distance % 2 == 0 {
                even_full += 1;
                if distance > 65 {
                    even_corners += 1;
                }
            } else {
                odd_full += 1;
                if distance > 65 {
                    odd_corners += 1;
                }
            }

            let neighbors = [
                (x.checked_sub(1), Some(y)),
                (Some(x + 1), Some(y)),
                (Some(x), y.checked_sub(1)),
                (Some(x), Some(y + 1)),
            ];
            neighbors
                .into_iter()
                .filter_map(|(x, y)| x.zip(y))
                .filter(|&(x, y)| x < side && y < side)
                .filter(|&(x, y)| !walls.contains((y * side + x) as u32))
                .for_each(|pos| q.push(pos, distance + 1))
        }

        let n = (P2_TOTAL_STEPS - (side / 2)) / side;
        let part2 = ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners + n * even_corners - n;

        (part1, part2)
}
