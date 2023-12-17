use std::fmt::Display;

// Adaptation of https://www.reddit.com/r/adventofcode/comments/18k9ne5/2023_day_17_solutions/kdqeywd/
// With the addition of bucket queue + my own optimizations

use ::bucket_queue::*;

const UP: u8 = 0;
const RIGHT: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 3;

struct Grid {
    costs: Box<[u8]>,
    columns: usize,
}

impl Grid {
    fn new(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let columns = lines.peek().map_or(0, |l| l.len());
        Self {
            costs: lines.flat_map(str::as_bytes).map(|&c| c - b'0').collect::<Box<_>>(),
            columns,
        }
    }

    fn step(&self, pos: usize, dir: u8) -> Option<usize> {
        let rows = self.costs.len();
        Some(match dir {
            UP if pos > self.columns => pos - self.columns,
            RIGHT if (pos + 1) % self.columns != 0 => pos + 1,
            DOWN if pos < rows - self.columns => pos + self.columns,
            LEFT if pos % self.columns != 0 => pos - 1,
            _ => return None,
        })
    }
}

fn do_solve(input: &str, min_straight_steps: usize, max_straight_steps: usize) -> Option<usize> {
    let grid = Grid::new(input);
    let start = 0;
    let goal = grid.costs.len() - 1;

    let mut visited = vec![0u8; grid.costs.len()];
    let mut cost_cache = vec![usize::MAX; 2 * grid.costs.len()];
    let mut q: BucketQueue<Vec<_>> = BucketQueue::new();

    q.push((start, UP), 0);
    q.push((start, RIGHT), 0);

    // Here's the idea: There's no meaningful difference between horizontal and vertical movement. At each step, we
    // can just go straight any allowed number of straight steps, then turn.

    while let Some(cost) = q.min_priority() {
        // Our state is: the current position and what direction we were facing when we got here.
        let (pos, previous_dir) = q.pop(cost).unwrap();

        // If this is the goal, great!
        if pos == goal {
            return Some(cost);
        }

        // If we've already visited this position from this direction, we can skip it.
        if visited[pos] & (1u8 << previous_dir) != 0 {
            continue;
        }
        visited[pos] |= 1u8 << previous_dir;

        let left_dir = previous_dir ^ 1;
        let right_dir = left_dir ^ 2;
        // For each possible turning direction...
        'turnloop: for turn_dir in [left_dir, right_dir] {
            let mut new_cost = cost;
            let mut new_pos = pos;

            // Go straight in the new direction the requisite amount of steps.
            for _ in 1..min_straight_steps {
                new_pos = if let Some(np) = grid.step(new_pos, turn_dir) {
                    np
                } else {
                    continue 'turnloop;
                };
                new_cost += grid.costs[new_pos] as usize;
            }

            // After we've exhausted the minimum number of straight steps, we can choose to stop walking and allow
            // another opportunity to turn.
            for _ in min_straight_steps..=max_straight_steps {
                new_pos = if let Some(np) = grid.step(new_pos, turn_dir) {
                    np
                } else {
                    break;
                };
                new_cost += grid.costs[new_pos] as usize;

                // Stop walking! We only add the state to the queue if it's better than what we've seen before.
                let cache_idx = (new_pos << 1) | left_dir as usize;
                if cost_cache[cache_idx] > new_cost {
                    cost_cache[cache_idx] = new_cost;
                    q.push((new_pos, left_dir), new_cost);
                }
            }
        }
    }

    None
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    rayon::join(|| do_solve(input, 0, 3).unwrap(), || do_solve(input, 4, 10).unwrap())
}
