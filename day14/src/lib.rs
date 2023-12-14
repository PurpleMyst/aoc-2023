use std::{fmt::Display};

use ahash::{HashMap, HashMapExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Sphere,
    Cube,
}

type Map = Vec<Vec<Cell>>;

const TOTAL_CYCLES: usize = 1_000_000_000;
const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn tilt_north(map: &mut Map) {
    let mut free_spots = [-1isize; WIDTH];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if map[y][x] == Cell::Sphere {
                free_spots[x] += 1;
                map[y][x] = Cell::Empty;
                map[free_spots[x] as usize][x] = Cell::Sphere;
            } else if map[y][x] == Cell::Cube {
                free_spots[x] = y as isize;
            }
        }
    }
}

fn tilt_south(map: &mut Map) {
    let mut free_spots = [HEIGHT; WIDTH];
    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            if map[y][x] == Cell::Sphere {
                free_spots[x] -= 1;
                map[y][x] = Cell::Empty;
                map[free_spots[x]][x] = Cell::Sphere;
            } else if map[y][x] == Cell::Cube {
                free_spots[x] = y;
            }
        }
    }
}

fn tilt_east(map: &mut Map) {
    map.iter_mut().for_each(|row| {
        let mut free_spot = WIDTH;
        for x in (0..WIDTH).rev() {
            if row[x] == Cell::Sphere {
                free_spot -= 1;
                row[x] = Cell::Empty;
                row[free_spot as usize] = Cell::Sphere;
            } else if row[x] == Cell::Cube {
                free_spot = x;
            }
        }
    });
}

fn tilt_west(map: &mut Map) {
    map.iter_mut().for_each(|row| {
        let mut free_spot = -1;
        for x in 0..WIDTH {
            if row[x] == Cell::Sphere {
                free_spot += 1;
                row[x] = Cell::Empty;
                row[free_spot as usize] = Cell::Sphere;
            } else if row[x] == Cell::Cube {
                free_spot = x as isize;
            }
        }
    });
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
    debug_assert_eq!(map[0].len(), WIDTH);
    debug_assert_eq!(map.len(), HEIGHT);

    let part1 = {
        let mut map = map.clone();
        tilt_north(&mut map);
        total_load(&map)
    };

    let mut cycle_nums = HashMap::new();
    let (cycles_before_loop, cycle_len) = (0..)
        .find_map(|cycle_num| {
            spin_cycle(&mut map);
            Some((cycle_num, cycle_num - cycle_nums.insert(map.clone(), cycle_num)?))
        })
        .unwrap();
    let cycles_remaining = (TOTAL_CYCLES - (cycles_before_loop + 1)) % cycle_len;
    for _ in 0..cycles_remaining {
        spin_cycle(&mut map);
    }

    let part2 = total_load(&map);

    (part1, part2)
}
