use std::fmt::Display;

use pathfinding::prelude::*;

// Forward in the directions array = rotate 90° counter-clockwise;
// Backward in the directions array = rotate 90° clockwise;
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    position: (usize, usize),
    direction: usize,
    steps_before_can_turn: usize,
    steps_before_must_turn: usize,
}

fn try_add_dir(pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
    Some((pos.0.checked_add_signed(dir.0)?, pos.1.checked_add_signed(dir.1)?))
}

fn do_solve(
    map: &Matrix<u64>,
    min_straight_steps: usize,
    max_straight_steps: usize,
    starting_dir: usize,
) -> Option<u64> {
    let goal = (map.columns() - 1, map.rows() - 1);

    let (_states, answer) = dijkstra(
        &State {
            position: (0, 0),
            direction: starting_dir,
            steps_before_must_turn: max_straight_steps - 1,
            steps_before_can_turn: min_straight_steps.saturating_sub(1),
        },
        |state| {
            let mut next = Vec::new();

            let current_dir = DIRECTIONS[state.direction];

            if state.steps_before_can_turn == 0 {
                let left_dir_idx = (state.direction + 1) % DIRECTIONS.len();
                let left_dir = DIRECTIONS[left_dir_idx];
                if let Some(left_pos) = try_add_dir(state.position, left_dir) {
                    if let Some(&cost) = map.get((left_pos.1, left_pos.0)) {
                        next.push((
                            State {
                                position: left_pos,
                                direction: left_dir_idx,
                                steps_before_must_turn: max_straight_steps - 1,
                                steps_before_can_turn: min_straight_steps.saturating_sub(1),
                            },
                            cost,
                        ));
                    }
                }

                let right_dir_idx = (state.direction + DIRECTIONS.len() - 1) % DIRECTIONS.len();
                let right_dir = DIRECTIONS[right_dir_idx];
                if let Some(right_pos) = try_add_dir(state.position, right_dir) {
                    if let Some(&cost) = map.get((right_pos.1, right_pos.0)) {
                        next.push((
                            State {
                                position: right_pos,
                                direction: right_dir_idx,
                                steps_before_must_turn: max_straight_steps - 1,
                                steps_before_can_turn: min_straight_steps.saturating_sub(1),
                            },
                            cost,
                        ));
                    }
                }
            }

            if let Some(straight_pos) = try_add_dir(state.position, current_dir) {
                if let Some(&cost) = map.get((straight_pos.1, straight_pos.0)) {
                    if let Some(steps_left) = state.steps_before_must_turn.checked_sub(1) {
                        next.push((
                            State {
                                position: straight_pos,
                                direction: state.direction,
                                steps_before_must_turn: steps_left,
                                steps_before_can_turn: state.steps_before_can_turn.saturating_sub(1),
                            },
                            cost,
                        ));
                    }
                }
            }

            next
        },
        |state| state.position == goal && state.steps_before_can_turn == 0,
    )?;

    Some(answer)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let map = include_str!("input.txt")
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').map(u64::from).collect::<Vec<_>>())
        .collect::<Matrix<_>>();

    let p1 = do_solve(&map, 0, 3, 1).unwrap();
    let p2 = (0..DIRECTIONS.len())
        .filter_map(|dir| do_solve(&map, 4, 10, dir))
        .min()
        .unwrap();
    (p1, p2)
}
