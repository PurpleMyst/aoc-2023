use std::{
    fmt::Display,
};

use ahash::{HashMap, HashSet, HashMapExt, HashSetExt};

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let mut symbols = HashSet::new();
    let mut gears = HashMap::new();
    input.lines().enumerate().for_each(|(y, row)| {
        row.bytes().enumerate().for_each(|(x, b)| {
            if !matches!(b, b'0'..=b'9' | b'.') {
                symbols.insert((x, y));
                gears.insert((x, y), Some((None, None)));
            }
        });
    });

    let mut part1 = 0;
    let mut nearby_gears = HashSet::new();

    for (y, row) in input.lines().enumerate() {
        let mut found_symbol_nearby = false;
        let mut number_so_far = 0;

        for (x, b) in row.bytes().enumerate() {
            if matches!(b, b'0'..=b'9') {
                number_so_far = number_so_far * 10 + (b - b'0') as u64;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if let (Some(x), Some(y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                            if symbols.contains(&(x, y)) {
                                found_symbol_nearby = true;
                                if gears.contains_key(&(x, y)) {
                                    nearby_gears.insert((x, y));
                                }
                            }
                        }
                    }
                }
            } else {
                if found_symbol_nearby {
                    part1 += number_so_far;
                    if number_so_far != 0 {
                        for (x, y) in nearby_gears.drain() {
                            match gears.get_mut(&(x, y)).unwrap() {
                                Some((l @ None, None)) => *l = Some(number_so_far),
                                Some((Some(_), r @ None)) => *r = Some(number_so_far),
                                otherwise => *otherwise = None,
                            }
                        }
                    }
                }
                found_symbol_nearby = false;
                number_so_far = 0;
            }
        }

        // don't forget the last number
        if found_symbol_nearby {
            part1 += number_so_far;

            if number_so_far != 0 {
                for (x, y) in nearby_gears.drain() {
                    match gears.get_mut(&(x, y)).unwrap() {
                        Some((l @ None, None)) => *l = Some(number_so_far),
                        Some((Some(_), r @ None)) => *r = Some(number_so_far),
                        otherwise => *otherwise = None,
                    }
                }
            }
        }
    }

    let part2 = gears
        .values()
        .filter_map(|&nearby| {
            let (l, r) = nearby?;
            Some(l? * r?)
        })
        .sum::<u64>();
    debug_assert!(part2 > 81890877, "{part2} is too low");

    (part1, part2)
}
