use std::{cmp::max, fmt::Display, mem::swap};

use grid::Grid;
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;

type Coordinate = u8;
type Position = (Coordinate, Coordinate);
type Map = Grid<Cell>;

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
    fn apply_to(self, pos: Position, coming_from: Direction, beams: &mut HashSet<(Position, Direction)>) {
        match (self, coming_from) {
            // Empty tiles or pointy ends just pass the beam through.
            (Empty, _)
            | (VerticalSplitter, Up)
            | (VerticalSplitter, Down)
            | (HorizontalSplitter, Left)
            | (HorizontalSplitter, Right) => {
                beams.insert((pos, coming_from));
            }

            // The mirrors reflect, obviously.
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

            // The splitters... split!
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
    /// Move the beam in this direction, returning the new position if we didn't underflow.
    fn apply_to(self, pos: Position) -> Option<Position> {
        match self {
            Up => Some((pos.0, pos.1.checked_sub(1)?)),
            Down => Some((pos.0, pos.1 + 1)),
            Left => Some((pos.0.checked_sub(1)?, pos.1)),
            Right => Some((pos.0 + 1, pos.1)),
        }
    }
}

fn do_solve(map: &Map, insertion_point: Position, initial_dir: Direction) -> usize {
    let mut beams = HashSet::default();
    let mut new_beams = HashSet::default();
    let mut seen_beams = Grid::<u8>::new(map.rows(), map.cols());

    // Account for the possibility that the insertion point is already some sort of special tile.
    map[(insertion_point.1 as usize, insertion_point.0 as usize)].apply_to(insertion_point, initial_dir, &mut beams);

    while !beams.is_empty() {
        for ((x, y), dir) in beams.drain() {
            // If there's already been a beam in this spot and moving in this direction, there's no need to recalculate.
            let pos_seen = &mut seen_beams[(y as usize, x as usize)];
            if (*pos_seen) & (1 << dir as u8) != 0 {
                continue;
            }
            *pos_seen |= 1 << dir as u8;

            // Apply the tile's effect and ensure we're still on the map.
            let Some(next_pos) = dir.apply_to((x, y)) else { continue };
            let Some(next_tile) = map.get(next_pos.1 as usize, next_pos.0 as usize) else {
                continue;
            };
            next_tile.apply_to(next_pos, dir, &mut new_beams);
        }
        swap(&mut beams, &mut new_beams);
    }

    seen_beams.iter().filter(|&&seen| seen != 0).count()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let columns = input.lines().next().unwrap().len();
    let map = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '.' => Empty,
                '/' => ForwardMirror,
                '\\' => BackMirror,
                '|' => VerticalSplitter,
                '-' => HorizontalSplitter,
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>();
    let map = Map::from_vec(map, columns);

    // Part 1 is simply one iteration of part 2.
    let part1 = do_solve(&map, (0, 0), Right);

    // Part 2 works by just... brute-forcing every possible starting position! :)
    let width = map.cols() as Coordinate;
    let height = map.rows() as Coordinate;
    let part2 = max(
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
