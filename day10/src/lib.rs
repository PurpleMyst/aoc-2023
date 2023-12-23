use std::fmt::Display;

use ::tap::Pipe as _;
use grid::Grid;

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
    let mut start_pos = None;
    let input = include_str!("input.txt");
    let columns = input.lines().next().unwrap().len();
    let map = input
        .bytes()
        .filter(|b| *b != b'\n')
        .enumerate()
        .map(|(idx, b)| match b {
            b'|' => Some(Pipe::NS),
            b'-' => Some(Pipe::EW),
            b'L' => Some(Pipe::NE),
            b'J' => Some(Pipe::NW),
            b'7' => Some(Pipe::SW),
            b'F' => Some(Pipe::SE),
            b'S' => {
                let y = idx / columns;
                let x = idx % columns;
                start_pos = Some((y, x));
                Some(S_REPLACEMENT)
            }
            b'.' => None,
            _ => unreachable!("{b:?}"),
        })
        .pipe(|iter| Grid::from_vec(iter.collect(), columns));

    let start_pos = start_pos.unwrap();
    let mut pos = start_pos;
    let mut dir: (isize, isize) = match S_REPLACEMENT {
        Pipe::EW => (0, 1),
        _ => unimplemented!(),
    };

    let mut loop_len = 0;
    let mut area = 0isize;

    loop {
        loop_len += 1;
        area -= dir.0 * pos.1 as isize; // Green's theorem

        match map[pos].expect("should not leave loop!") {
            // Straight segments just leave the direction as is
            Pipe::NS | Pipe::EW => {}

            // Curved segments change the direction
            Pipe::NE => {
                // ╚
                dir = match dir {
                    (1, 0) => (0, 1),   // down -> right
                    (0, -1) => (-1, 0), // left -> up
                    _ => unreachable!(),
                }
            }
            Pipe::NW => {
                // ╝
                dir = match dir {
                    (0, 1) => (-1, 0), // right -> up
                    (1, 0) => (0, -1), // down -> left
                    _ => unreachable!("{dir:?}"),
                }
            }
            Pipe::SW => {
                // ╗
                dir = match dir {
                    (0, 1) => (1, 0),   // right -> down
                    (-1, 0) => (0, -1), // up -> left
                    _ => unreachable!(),
                }
            }
            Pipe::SE => {
                // ╔
                dir = match dir {
                    (-1, 0) => (0, 1), // up -> right
                    (0, -1) => (1, 0), // right -> down
                    _ => unreachable!(),
                }
            }
        }

        pos.0 = pos.0.checked_add_signed(dir.0).unwrap();
        pos.1 = pos.1.checked_add_signed(dir.1).unwrap();

        if pos == start_pos {
            break;
        }
    }

    let part1 = loop_len / 2;
    let part2 = area.abs() - part1 + 1;
    (part1, part2)
}
