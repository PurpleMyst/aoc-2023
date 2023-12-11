use std::fmt::Display;
fn do_solve(padding: usize) -> usize {
    // Load the original space.
    let space = (include_str!("input.txt"))
        .lines()
        .map(|row| row.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Find which rows need to be expanded.
    let mut empty_row_indices = space
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| !c))
        .map(|(y, _)| y)
        .peekable();
    let empty_col_indices = (0..space[0].len())
        .filter(|&x| space.iter().all(|row| !row[x]))
        .collect::<Vec<_>>();

    // Expand the space! Each point after an expansion is offset by the number of
    // empty rows/cols before it times the padding.
    let mut vertices = Vec::new();
    let mut y_offset = 0;
    for (y, row) in space.iter().enumerate() {
        if empty_row_indices.peek().filter(|&&ey| ey == y).is_some() {
            y_offset += padding;
            empty_row_indices.next();
            continue;
        }

        let mut empty_col_indices = empty_col_indices.iter().copied().peekable();
        let mut x_offset = 0;
        for (x, &cell) in row.iter().enumerate() {
            if empty_col_indices.peek().filter(|&&ex| ex == x).is_some() {
                x_offset += padding;
                empty_col_indices.next();
                continue;
            }
            if !cell {
                continue;
            };
            vertices.push((x + x_offset, y + y_offset));
        }
    }

    // Find the Manhattan distance between each pair of vertices.
    let mut answer = 0;
    for (i, &start) in vertices.iter().enumerate() {
        for &goal in vertices.iter().skip(i + 1) {
            answer += start.0.abs_diff(goal.0) + start.1.abs_diff(goal.1);
        }
    }
    answer
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    (do_solve(1), do_solve(1_000_000 - 1))
}
