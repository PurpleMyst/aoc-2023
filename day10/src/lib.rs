use std::fmt::Display;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::NS => write!(f, "|"),
            Pipe::EW => write!(f, "-"),
            Pipe::NE => write!(f, "L"),
            Pipe::NW => write!(f, "J"),
            Pipe::SW => write!(f, "7"),
            Pipe::SE => write!(f, "F"),
        }
    }
}

const S_REPLACEMENT: Pipe = Pipe::EW;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut start_pos = None;

    let map = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '|' => Some(Pipe::NS),
                    '-' => Some(Pipe::EW),
                    'L' => Some(Pipe::NE),
                    'J' => Some(Pipe::NW),
                    '7' => Some(Pipe::SW),
                    'F' => Some(Pipe::SE),
                    'S' => {
                        start_pos = Some((x, y));
                        Some(S_REPLACEMENT)
                    }
                    '.' => None,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut q = vec![(start_pos.unwrap(), 0)];
    let mut distance_map = vec![vec![usize::MAX; map[0].len()]; map.len()];
    while let Some((n, d)) = q.pop() {
        let best = &mut distance_map[n.1][n.0];
        if *best < d {
            continue;
        }
        *best = d;

        let pipe = map[n.1][n.0];
        let neighbors = match pipe.unwrap() {
            Pipe::NS => [(n.0, n.1 - 1), (n.0, n.1 + 1)],
            Pipe::EW => [(n.0 + 1, n.1), (n.0 - 1, n.1)],
            Pipe::NE => [(n.0 + 1, n.1), (n.0, n.1 - 1)],
            Pipe::NW => [(n.0, n.1 - 1), (n.0 - 1, n.1)],
            Pipe::SW => [(n.0 - 1, n.1), (n.0, n.1 + 1)],
            Pipe::SE => [(n.0, n.1 + 1), (n.0 + 1, n.1)],
        };
        q.extend_from_slice(&neighbors.map(|n| (n, d + 1)).as_slice());
    }

    let part1 = distance_map
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&d| d != usize::MAX)
        .max()
        .copied()
        .unwrap();

    let mut left = usize::MAX;
    let mut right = 0;
    let mut top = usize::MAX;
    let mut bottom = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if distance_map[y][x] == usize::MAX {
                continue;
            }
            left = left.min(x);
            right = right.max(x);
            top = top.min(y);
            bottom = bottom.max(y);
        }
    }

    let candidates = (left..=right)
        .flat_map(|x| (top..=bottom).map(move |y| (x, y)))
        .filter(|&(x, y)| distance_map[y][x] == usize::MAX);

    let width = right - left + 1;
    let height = bottom - top + 1;

    // map of exploded pipes
    // means every pipe + its neighbors
    // padded with empty space all around
    let mut exploded_map = vec![vec![false; 2 * width - 1 + 2]; 2 * height - 1 + 2];

    for y in top..=bottom {
        for x in left..=right {
            if distance_map[y][x] != usize::MAX {
                let pipe = map[y][x].unwrap();

                let x = 2 * (x - left) + 1;
                let y = 2 * (y - top) + 1;
                exploded_map[y][x] = true;

                let neighbors = match pipe {
                    Pipe::NS => [(x, y - 1), (x, y + 1)],
                    Pipe::EW => [(x + 1, y), (x - 1, y)],
                    Pipe::NE => [(x + 1, y), (x, y - 1)],
                    Pipe::NW => [(x, y - 1), (x - 1, y)],
                    Pipe::SW => [(x - 1, y), (x, y + 1)],
                    Pipe::SE => [(x, y + 1), (x + 1, y)],
                };
                for (nx, ny) in neighbors {
                    if let Some(cell) = exploded_map.get_mut(ny).and_then(|row| row.get_mut(nx)) {
                        *cell = true;
                    }
                }
            }
        }
    }

    // x' = 2 * (x - left) + 1 => x = (x' - 1) / 2 + left
    let part2 = candidates
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter(|(x, y)| {
            let mut q = vec![(2 * (x - left) + 1, 2 * (y - top) + 1)];

            let mut visited = HashSet::new();

            while let Some((x, y)) = q.pop() {
                if x == 0
                    || y == 0
                    || !(left..=right).contains(&((x - 1) / 2 + left))
                    || !(top..=bottom).contains(&((y - 1) / 2 + top))
                {
                    return false;
                }
                if exploded_map.get(y).and_then(|row| row.get(x)).copied().unwrap_or(false) || !visited.insert((x, y)) {
                    continue;
                }

                let neighbors = [
                    (Some(x), y.checked_sub(1)),
                    (Some(x), Some(y + 1)),
                    (Some(x + 1), Some(y)),
                    (x.checked_sub(1), Some(y)),
                    (Some(x + 1), y.checked_sub(1)),
                    (x.checked_sub(1), y.checked_sub(1)),
                    (Some(x + 1), Some(y + 1)),
                    (x.checked_sub(1), Some(y + 1)),
                ];
                q.extend(neighbors.iter().filter_map(|&(x, y)| Some((x?, y?))));
            }

            true
        })
        .count();

    (part1, part2)
}
