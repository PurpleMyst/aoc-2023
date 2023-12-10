use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

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
    let mut distance_map = HashMap::<_, usize>::new();
    while let Some((n, d)) = q.pop() {
        let best = distance_map.entry(n).or_insert(d);
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

    let part1 = distance_map.values().max().copied().unwrap();

    let mut left = usize::MAX;
    let mut right = 0;
    let mut top = usize::MAX;
    let mut bottom = 0;
    for (x, y) in distance_map.keys() {
        left = left.min(*x);
        right = right.max(*x);
        top = top.min(*y);
        bottom = bottom.max(*y);
    }

    let candidates = (left..=right)
        .flat_map(|x| (top..=bottom).map(move |y| (x, y)))
        .filter(|&(x, y)| !distance_map.contains_key(&(x, y)));

    let width = right - left + 1;
    let height = bottom - top + 1;

    // map of exploded pipes
    // means every pipe + its neighbors
    // padded with empty space all around
    let mut exploded_map = vec![vec![false; 2 * width - 1 + 2]; 2 * height - 1 + 2];
    let mut inside = HashSet::new();

    for &(x, y) in distance_map.keys() {
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

    // print out exploded map
    // x' = 2 * (x - left) + 1 => x = (x' - 1) / 2 + left

    let mut part2 = 0;
    for (x, y) in candidates {
        // if (x, y) != (5, 3){
        //     continue;
        // }

        let mut q = vec![(2 * (x - left) + 1, 2 * (y - top) + 1)];
        let mut visited = HashSet::new();
        let mut broke = false;

        while let Some((x, y)) = q.pop() {
            // println!("{x},{y}");
            if x == 0
                || y == 0
                || !(left..=right).contains(&((x - 1) / 2 + left))
                || !(top..=bottom).contains(&((y - 1) / 2 + top))
            {
                // dbg!();
                broke = true;
                break;
            }
            if exploded_map.get(y).and_then(|row| row.get(x)).copied().unwrap_or(false) {
                // dbg!();
                continue;
            }
            if !visited.insert((x, y)) {
                // dbg!();
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

        // dbg!(visited.len(), broke);

        if broke {
            // for (yp, row) in exploded_map.iter().enumerate() {
            //     for (xp, cell) in row.iter().enumerate() {
            //         if (xp,yp) == (2*(x-left),2*(y-top))  {
            //             print!("\x1b[1;32m{}\x1b[0m", 'K');
            //         }
            //         else if visited.contains(&(xp, yp)) {
            //             print!("\x1b[1;31m{}\x1b[0m", '*');
            //         }
            //         else if xp % 2 == 0 && yp % 2 == 0 {
            //             let x = xp / 2 + left;
            //             let y = yp / 2 + top;
            //             if let Some(pipe) = map[y][x] {
            //                 print!("\x1b[1m{}\x1b[0m", pipe);
            //             } else {
            //                 print!(".");
            //             }
            //         } else {
            //             print!("{}", if *cell { '\u{2588}' } else { '.' });
            //         }
            //     }
            //     println!();
            // }
        } else {
            part2 += 1;
            // TODO: inside.extend(visited)?
            inside.insert((x, y));
        }

        // break;
    }

    // print exploded map
    println!("EXPLODED:");
    for (yp, row) in exploded_map.iter().enumerate() {
        for (xp, cell) in row.iter().enumerate() {
            if xp % 2 == 1 && yp % 2 == 1 {
                let x = (xp - 1) / 2 + left;
                let y = (yp - 1) / 2 + top;
                if inside.contains(&(x, y)) {
                    print!("\x1b[1;31m{}\x1b[0m", 'I');
                } else if let Some(pipe) = map[y][x] {
                    print!(
                        "\x1b[{}m{}\x1b[0m",
                        if distance_map.contains_key(&(x, y)) { "35" } else { "2" },
                        pipe
                    );
                } else {
                    print!("\x1b[32mO\x1b[0m");
                }
            } else {
                print!("{}", if *cell { '\u{2588}' } else { '.' });
            }
        }
        println!();
    }

    // print non exploded
    println!();
    println!("NON-EXPLODED:");
    for y in top..=bottom {
        for x in left..=right {
            if inside.contains(&(x, y)) {
                print!("\x1b[1;31m{}\x1b[0m", 'I');
            } else if let Some(pipe) = map[y][x] {
                print!("\x1b[1m{}\x1b[0m", pipe);
            } else {
                print!("\x1b[32mO\x1b[0m");
            }
        }
        println!();
    }

    (part1, part2)
}
