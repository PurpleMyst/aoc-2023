use std::fmt::Display;

use ahash::{HashSet, HashSetExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Sphere,
    Cube,
}

type Map = Vec<Vec<Cell>>;

const TOTAL_CYCLES: usize = 1_000_000_000;

fn tilt_north(map: &mut Map) {
    loop {
        let mut moved = false;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == Cell::Sphere && y != 0 && map[y - 1][x] == Cell::Empty {
                    map[y][x] = Cell::Empty;
                    map[y - 1][x] = Cell::Sphere;
                    moved = true;
                }
            }
        }
        if !moved {
            break;
        }
    }
}

fn tilt_south(map: &mut Map) {
    loop {
        let mut moved = false;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == Cell::Sphere && y != map.len() - 1 && map[y + 1][x] == Cell::Empty {
                    map[y][x] = Cell::Empty;
                    map[y + 1][x] = Cell::Sphere;
                    moved = true;
                }
            }
        }
        if !moved {
            break;
        }
    }
}

fn tilt_east(map: &mut Map) {
    loop {
        let mut moved = false;

        for row in map.iter_mut() {
            for x in 0..row.len() {
                if row[x] == Cell::Sphere && x != row.len() - 1 && row[x + 1] == Cell::Empty {
                    row[x] = Cell::Empty;
                    row[x + 1] = Cell::Sphere;
                    moved = true;
                }
            }
        }
        if !moved {
            break;
        }
    }
}

fn tilt_west(map: &mut Map) {
    loop {
        let mut moved = false;

        for row in map.iter_mut() {
            for x in 0..row.len() {
                if row[x] == Cell::Sphere && x != 0 && row[x - 1] == Cell::Empty {
                    row[x] = Cell::Empty;
                    row[x - 1] = Cell::Sphere;
                    moved = true;
                }
            }
        }
        if !moved {
            break;
        }
    }
}

fn total_load(map: &Map) -> usize {
    let height = map.len();
    map.iter()
        .zip((1..=height).rev())
        .map(|(row, weight)| row.iter().filter(|&&cell| cell == Cell::Sphere).count() * weight)
        .sum()
}

fn spin_cycle(map: &mut Map) {
    tilt_north(map);
    tilt_west(map);
    tilt_south(map);
    tilt_east(map);
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let mut map = input
        .lines()
        .map(|row| {
            row.bytes()
                .map(|b| match b {
                    b'.' => Cell::Empty,
                    b'O' => Cell::Sphere,
                    b'#' => Cell::Cube,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = {
        let mut map = map.clone();
        tilt_north(&mut map);
        total_load(&map)
    };

    let mut visited = HashSet::new();
    let cycles_before_loop = (0..)
        .find(|_| {
            spin_cycle(&mut map);
            !visited.insert(map.clone())
        })
        .unwrap()
        + 1;
    let mut map2 = map.clone();
    let cycle_len = (1..)
        .find(|_| {
            spin_cycle(&mut map2);
            map2 == map
        })
        .unwrap();
    let cycles_remaining = (TOTAL_CYCLES - cycles_before_loop) % cycle_len;
    for _ in 0..cycles_remaining {
        spin_cycle(&mut map);
    }

    let part2 = total_load(&map);

    (part1, part2)
}
