use std::{
    fmt::Display,
};

use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Cell {
    Empty,
    ForwardMirror,
    BackMirror,
    VerticalSplitter,
    HorizontalSplitter,
}
use Cell::*;

impl Cell {
    fn apply_to(self, pos: (usize, usize), coming_from: Direction, beams: &mut HashSet<((usize, usize), Direction)>) {
        match (self, coming_from) {
            (Empty, _)
            | (VerticalSplitter, Up)
            | (VerticalSplitter, Down)
            | (HorizontalSplitter, Left)
            | (HorizontalSplitter, Right) => {
                beams.insert((pos, coming_from));
            }

            (ForwardMirror, Right) | (BackMirror, Left) => {
                beams.insert((pos, Up));
            }

            (ForwardMirror, Up) | (BackMirror, Down) => {
                beams.insert((pos, Right));
            }

            (ForwardMirror, Left) | (BackMirror, Right) => {
                beams.insert((pos, Down));
            }

            (ForwardMirror, Down) | (BackMirror, Up) => {
                beams.insert((pos, Left));
            }

            (VerticalSplitter, Left) | (VerticalSplitter, Right) => {
                beams.insert((pos, Up));
                beams.insert((pos, Down));
            }

            (HorizontalSplitter, Up) | (HorizontalSplitter, Down) => {
                beams.insert((pos, Left));
                beams.insert((pos, Right));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl Direction {
    fn apply_to(self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Up => Some((pos.0, pos.1.checked_sub(1)?)),
            Down => Some((pos.0, pos.1 + 1)),
            Left => Some((pos.0.checked_sub(1)?, pos.1)),
            Right => Some((pos.0 + 1, pos.1)),
        }
    }
}

fn count_visited(visited: &Vec<Vec<bool>>) -> usize {
    visited
        .iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum::<usize>()
}

fn do_solve(map: &Vec<Vec<Cell>>, insertion_point: (usize, usize), initial_dir: Direction) -> usize {
    let mut beams = HashSet::default();
    let mut new_beams = HashSet::default();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut seen_beams = HashSet::default();

    map[insertion_point.1][insertion_point.0].apply_to(insertion_point, initial_dir, &mut beams);

    while seen_beams.insert(beams.iter().copied().collect::<Vec<_>>()) {
        for ((x, y), dir) in beams.drain() {
            visited[y][x] = true;
            let Some(next_pos) = dir.apply_to((x, y)) else { continue };
            let Some(next_tile) = map.get(next_pos.1).and_then(|row| row.get(next_pos.0)) else {
                continue;
            };
            next_tile.apply_to(next_pos, dir, &mut new_beams);
        }
        std::mem::swap(&mut beams, &mut new_beams);
    }

    count_visited(&visited)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let map = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Empty,
                    '/' => ForwardMirror,
                    '\\' => BackMirror,
                    '|' => VerticalSplitter,
                    '-' => HorizontalSplitter,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = do_solve(&map, (0, 0), Right);

    let width = map[0].len();
    let height = map.len();
    let part2 = std::cmp::max(
        (0..width)
            .into_par_iter()
            .map(|x| do_solve(&map, (x, 0), Down).max(do_solve(&map, (x, height - 1), Up)))
            .max()
            .unwrap(),
        (0..height)
            .into_par_iter()
            .map(|y| do_solve(&map, (0, y), Right).max(do_solve(&map, (width - 1, y), Left)))
            .max()
            .unwrap(),
    );

    (part1, part2)
}
