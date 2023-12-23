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
    steps: i64,
}

// Solve using Pick's theorem.
fn do_solve(instructions: &[Instruction]) -> i64 {
    // Calculate the vertices by following the instructions. Along the way, it's easy to calculate the perimeter.
    let mut x = 0;
    let mut y = 0;
    let mut vertices = vec![(0, 0)];
    vertices.reserve(instructions.len() + 1);
    let mut perimeter = 0;
    for &Instruction { direction, steps } in instructions {
        let destination = match direction {
            Direction::Up => (x, y + steps),
            Direction::Down => (x, y - steps),
            Direction::Left => (x - steps, y),
            Direction::Right => (x + steps, y),
        };
        (x, y) = destination;
        vertices.push(destination);
        perimeter += steps;
    }

    // Utilize the shoelace formula to calculate the area, taking the absolute value to avoid depending on the order of
    // the vertices.
    let area = vertices
        .windows(2)
        .map(|w| {
            let (x1, y1) = w[0];
            let (x2, y2) = w[1];
            x1 * y2 - x2 * y1
        })
        .sum::<i64>()
        .abs()
        / 2;

    // Pick's theorem states that:
    // Area = InteriorIntegerPoints + BoundaryIntegerPoints / 2 - 1
    // This means that we can calculate the number of interior integer points by:
    // InteriorIntegerPoints = Area - BoundaryIntegerPoints / 2 + 1
    // However, to that we must add the perimeter, as the problem description asks us to count the points on the boundary as well.
    // Therefore, the final formula is:
    area + perimeter / 2 + 1
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let (part1_instructions, part2_instructions): (Vec<_>, Vec<_>) = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut it = line.split(' ');
            let p1_direction = match it.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };
            let p1_steps = it.next().unwrap().parse().unwrap();

            let color = &it.next().unwrap()[2..8];
            let (p2_steps, p2_direction) = color.split_at(color.len() - 1);
            let p2_steps = u32::from_str_radix(p2_steps, 16).unwrap();
            let p2_direction = match p2_direction {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => unreachable!(),
            };

            (
                Instruction {
                    direction: p1_direction,
                    steps: p1_steps,
                },
                Instruction {
                    direction: p2_direction,
                    steps: p2_steps as _,
                },
            )
        })
        .unzip();

    (do_solve(&part1_instructions), do_solve(&part2_instructions))
}
