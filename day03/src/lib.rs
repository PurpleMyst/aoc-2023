use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input: &str = include_str!("input.txt");

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut symbols = vec![false; width * height];
    let mut gears = vec![None; width * height];
    input.lines().enumerate().for_each(|(y, row)| {
        row.bytes().enumerate().for_each(|(x, b)| {
            if !matches!(b, b'0'..=b'9' | b'.') {
                symbols[y * width + x] = true;
                if b == b'*' {
                    // gears.insert((x, y), Some((None, None)));
                    gears[y * width + x] = Some((None, None));
                };
            }
        });
    });

    let mut part1 = 0;
    let mut nearby_gears = Vec::new();

    for (y, row) in input.lines().enumerate() {
        let mut found_symbol_nearby = false;
        let mut number_so_far = 0;

        for (x, b) in row.bytes().enumerate() {
            if matches!(b, b'0'..=b'9') {
                number_so_far = number_so_far * 10 + (b - b'0') as u64;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if let (Some(x), Some(y)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                            if symbols.get(y * width + x).copied().unwrap_or_default() {
                                found_symbol_nearby = true;
                                if gears[y * width + x].is_some() {
                                    nearby_gears.push((x, y));
                                }
                            }
                        }
                    }
                }
            } else {
                if found_symbol_nearby {
                    part1 += number_so_far;
                    if number_so_far != 0 {
                        nearby_gears.sort_unstable();
                        nearby_gears.dedup();
                        for (x, y) in nearby_gears.drain(..) {
                            match &mut gears[y * width + x] {
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
                nearby_gears.sort_unstable();
                nearby_gears.dedup();
                for (x, y) in nearby_gears.drain(..) {
                    match gears.get_mut(y * width + x).unwrap() {
                        Some((l @ None, None)) => *l = Some(number_so_far),
                        Some((Some(_), r @ None)) => *r = Some(number_so_far),
                        otherwise => *otherwise = None,
                    }
                }
            }
        }
    }

    let part2 = gears
        .into_iter()
        .filter_map(|nearby| {
            let (l, r) = nearby?;
            Some(l? * r?)
        })
        .sum::<u64>();

    (part1, part2)
}
