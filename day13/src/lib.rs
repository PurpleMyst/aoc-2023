use std::fmt::Display;

fn count_diffs(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
}

fn show_vertical_reflection(map: &str, x: usize) {
    let rows: Vec<&str> = map.lines().collect();
    let _width = rows[0].len();
    let height = rows.len();
    let mut out = String::with_capacity(map.len() + height + 1);
    for y in 0..height {
        out.push_str(&rows[y][..x]);
        out.push('┋');
        out.push_str(&rows[y][x..]);
        out.push('\n');
    }
    let out = out.replace(".", "░").replace("#", "█");
    println!("{out}\n");
}

fn show_horizontal_reflection(map: &str, y: usize) {
    let rows: Vec<&str> = map.lines().collect();
    let width = rows[0].len();
    let height = rows.len();
    let mut out = String::with_capacity(map.len() + width + 1);
    for y in 0..y {
        out.push_str(rows[y]);
        out.push('\n');
    }
    out.push_str("┈".repeat(width).as_str());
    out.push('\n');
    for y in y..height {
        out.push_str(rows[y]);
        out.push('\n');
    }
    let out = out.replace(".", "░").replace("#", "█");
    println!("{out}\n");
}

fn process_map(map: &str, tolerance: usize) -> usize {
    println!("ORIGINAL MAP");
    println!("{}\n", map.replace(".", "░").replace("#", "█"));
    println!("REFLECTED MAP");

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
            == tolerance
        {
            show_horizontal_reflection(map, y);

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
            == tolerance
        {
            show_vertical_reflection(map, x);
            return x;
        }
    }

    unreachable!()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt").split("\n\n");

    let part1 = input.clone().map(|map| process_map(map, 0)).sum::<usize>();
    dbg!();
    println!("=============PART 2 START=========================");

    let part2 = input.map(|map| process_map(map, 1)).sum::<usize>();
    debug_assert!(
        part2 > 10_000 && part2 < 35_975,
        "{part2} not in correct range! (part1 was {part1})"
    );

    (part1, part2)
}
