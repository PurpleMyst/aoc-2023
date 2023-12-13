use std::fmt::Display;

fn count_diffs(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
}

fn process_map(map: &str, smudges: usize) -> usize {
    // check horizontal line of reflection
    let rows: Vec<&str> = map.lines().collect();
    for y in 1..rows.len() {
        let mut before = &rows[..y];
        let mut after = &rows[y..];
        if before.len() < after.len() {
            after = &after[..before.len()];
        } else {
            before = &before[before.len() - after.len()..];
        }
        if before
            .iter()
            .rev()
            .zip(after.iter())
            .map(|(a, b)| count_diffs(a, b))
            .sum::<usize>()
            == smudges
        {
            return 100 * y;
        }
    }

    // check vertical line of reflection
    let width = rows[0].len();
    for x in 1..width {
        if (0..rows.len())
            .map(|y| {
                let mut before = &rows[y][..x];
                let mut after = &rows[y][x..];
                if before.len() < after.len() {
                    after = &after[..before.len()];
                } else {
                    before = &before[before.len() - after.len()..];
                }
                before.chars().rev().zip(after.chars()).filter(|(a, b)| a != b).count()
            })
            .sum::<usize>()
            == smudges
        {
            return x;
        }
    }

    unreachable!()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt").split("\n\n");

    let part1 = input.clone().map(|map| process_map(map, 0)).sum::<usize>();

    let part2 = input.map(|map| process_map(map, 1)).sum::<usize>();
    debug_assert!(
        part2 > 10_000 && part2 < 35_975,
        "{part2} not in correct range! (part1 was {part1})"
    );

    (part1, part2)
}
