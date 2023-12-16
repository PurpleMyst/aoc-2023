use std::fmt::Display;

// use fxhash::{HashMap, HashMapExt};
use rustc_hash::FxHashMap as HashMap;

const TOTAL_CYCLES: usize = 1_000_000_000;
const WIDTH: usize = 100;
const HEIGHT: usize = 100;

const FIRST_COL: u128 = 1;
const LAST_COL: u128 = 1 << (WIDTH - 1);

// Implementation using u128 inspired by:
// https://reddit.com/r/adventofcode/comments/18i68p9/2023_day_14_avenues_for_further_optimization/kdet006/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Map {
    blocks: [u128; HEIGHT],
    rollers: [u128; HEIGHT],
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.rollers[y] & (1 << x) != 0 {
                    write!(f, "O")?;
                } else if self.blocks[y] & (1 << x) != 0 {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn total_load(&self) -> u32 {
        self.rollers
            .iter()
            .zip((1..=HEIGHT as u32).rev())
            .map(|(row, weight)| row.count_ones() * weight)
            .sum()
    }

    fn tilt_north(&mut self) {
        loop {
            let mut moved = false;
            for y in (1..HEIGHT).rev() {
                let blockers = self.rollers[y - 1] | self.blocks[y - 1];
                let north_rollers = self.rollers[y] & !blockers;
                if north_rollers != 0 {
                    moved = true;
                    self.rollers[y] &= !north_rollers;
                    self.rollers[y - 1] |= north_rollers;
                }
            }
            if !moved {
                break;
            }
        }
    }

    fn tilt_south(&mut self) {
        loop {
            let mut moved = false;
            for y in 0..(HEIGHT - 1) {
                let blockers = self.rollers[y + 1] | self.blocks[y + 1];
                let south_rollers = self.rollers[y] & !blockers;
                if south_rollers != 0 {
                    moved = true;
                    self.rollers[y] &= !south_rollers;
                    self.rollers[y + 1] |= south_rollers;
                }
            }
            if !moved {
                break;
            }
        }
    }

    // NB: LSB = first in row.
    // therefore, right shift = west shift.
    fn tilt_west(&mut self) {
        for (rollers, blocks) in self.rollers.iter_mut().zip(self.blocks) {
            loop {
                let blockers = *rollers | blocks;
                let west_rollers = *rollers & !(blockers << 1) & !FIRST_COL;
                if west_rollers != 0 {
                    *rollers &= !west_rollers;
                    *rollers |= west_rollers >> 1;
                } else {
                    break;
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for (rollers, blocks) in self.rollers.iter_mut().zip(self.blocks) {
            loop {
                let blockers = *rollers | blocks;
                let west_rollers = *rollers & !(blockers >> 1) & !LAST_COL;
                if west_rollers != 0 {
                    *rollers &= !west_rollers;
                    *rollers |= west_rollers << 1;
                } else {
                    break;
                }
            }
        }
    }

    fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut map = Map {
        blocks: [0; WIDTH],
        rollers: [0; WIDTH],
    };

    include_str!("input.txt")
        .lines()
        .zip(map.blocks.iter_mut().zip(map.rollers.iter_mut()))
        .for_each(|(row, (blocks, rollers))| {
            row.bytes().enumerate().for_each(|(i, b)| match b {
                b'O' => *rollers |= 1 << i,
                b'#' => *blocks |= 1 << i,
                b'.' => (),
                _ => unreachable!(),
            })
        });

    let part1 = {
        let mut p1_map = map;
        p1_map.tilt_north();
        p1_map.total_load()
    };

    let mut cycle_nums = HashMap::default();
    let mut last = 0;
    let (cycles_before_loop, cycle_len) = (1..)
        .find_map(|cycle_num| {
            map.spin_cycle();
            let load = map.total_load();
            let key = (load, last);
            last = load;
            Some((cycle_num, cycle_num - cycle_nums.insert(key, cycle_num)?))
        })
        .unwrap();
    let cycles_remaining = (TOTAL_CYCLES - cycles_before_loop) % cycle_len;
    for _ in 0..cycles_remaining {
        map.spin_cycle();
    }

    let part2 = map.total_load();

    (part1, part2)
}
