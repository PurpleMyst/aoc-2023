use ::bucket_queue::*;
use hibitset::BitSet;
use rustc_hash::FxHashSet as HashSet;

pub(crate) const TOTAL_STEPS: usize = 64;

pub(crate) fn solve(map: &BitSet, side: usize, start_pos: (usize, usize), total_steps: usize) -> usize {
    let mut q = BucketQueue::<Vec<_>>::new();
    q.push(start_pos, total_steps);
    let mut reachable = 0;
    let mut visited = HashSet::default();

    while let Some(remaining_steps) = q.max_priority() {
        let (x, y) = q.pop(remaining_steps).unwrap();
        let can_be_reached = remaining_steps % 2 == 0;
        if !visited.insert((x, y)) {
            continue;
        }

        if can_be_reached {
            reachable += 1;
        }

        if remaining_steps == 0 {
            continue;
        }

        let neighbors = [
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (Some(x), Some(y + 1)),
        ];
        neighbors
            .into_iter()
            .filter_map(|(x, y)| x.zip(y))
            .filter(|&(x, y)| x < side && y < side)
            .filter(|&(x, y)| !map.contains((y * side + x) as u32))
            .for_each(|pos| q.push(pos, remaining_steps - 1))
    }

    reachable
}
