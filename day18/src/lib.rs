use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    length: i64,
}

// Solve using Pick's theorem.
fn do_solve(instructions: &[Instruction]) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut vertices = vec![(0, 0)];
    vertices.reserve(instructions.len() + 1);
    let mut perimeter = 0;
    for &Instruction { direction, length } in instructions {
        let destination = match direction {
            Direction::Up => (x, y + length),
            Direction::Down => (x, y - length),
            Direction::Left => (x - length, y),
            Direction::Right => (x + length, y),
        };
        (x, y) = destination;
        vertices.push(destination);
        perimeter += length;
    }

    let area = vertices
        .windows(2)
        .map(|w| {
            let [(x1, y1), (x2, y2)] = w.try_into().unwrap();
            x1 * y2 - x2 * y1
        })
        .sum::<i64>() / 2;

    // Area = Interior + Boundary / 2 - 1
    // => Interior = Area - Boundary / 2 + 1
    let interior = area - perimeter / 2 + 1;

    // Zero idea why this is needed.
    let answer = 2 - interior;

    answer
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (part1_instructions, part2_instructions): (Vec<_>, Vec<_>) = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut it = line.split(' ');
            let direction = match it.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };
            let amount = it.next().unwrap().parse().unwrap();

            let color = &it.next().unwrap()[2..8];
            let (c_amount, c_direction) = color.split_at(color.len() - 1);
            let c_amount = u32::from_str_radix(c_amount, 16).unwrap();
            let c_direction = match c_direction {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => unreachable!(),
            };

            (
                Instruction { direction, length: amount },
                Instruction {
                    direction: c_direction,
                    length: c_amount as _,
                },
            )
        })
        .unzip();

    (do_solve(&part1_instructions), do_solve(&part2_instructions))
}
