use std::fmt::Display;

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
            Pipe::NS => write!(f, "║"),
            Pipe::EW => write!(f, "═"),
            Pipe::NE => write!(f, "╚"),
            Pipe::NW => write!(f, "╝"),
            Pipe::SW => write!(f, "╗"),
            Pipe::SE => write!(f, "╔"),
        }
    }
}

const S_REPLACEMENT: Pipe = Pipe::EW;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    // Load in the map, replacing our starting position with a pipe. We verify which one is the
    // correct replacement by visual inspection.
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

    // Simple depth-first search to find points in the main loop.
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

    // Find the furthest point from the start.
    let part1 = distance_map
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&d| d != usize::MAX)
        .max()
        .copied()
        .unwrap();

    // Parity algorithm: Every time we hit a pipe (that's part of the main loop) going up (be it
    // vertically or diagonally), we flip a boolean that indicates if we're inside or outside the
    // main loop.
    let part2 = map
        .into_iter()
        .zip(distance_map.into_iter())
        .map(|(map_row, dist_row)| {
            let mut inside = false;
            let mut counter = 0;
            for (cell, dist) in map_row.into_iter().zip(dist_row.into_iter()) {
                let Some(pipe) = cell.filter(|_| dist != usize::MAX) else {
                    if inside {
                        counter += 1;
                    }
                    continue;
                };

                if matches!(pipe, Pipe::NS | Pipe::SE | Pipe::SW) {
                    inside = !inside;
                }
            }
            counter
        })
        .sum::<usize>();

    (part1, part2)
}
