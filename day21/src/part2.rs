use ::bucket_queue::{BucketQueue, LastInFirstOutQueue, Queue};
use grid::Grid;

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub(crate) const TOTAL_STEPS: usize = 26_501_365;

pub(crate) struct InfiniteGrid {
    pub(crate) contents: Grid<bool>,
    pub(crate) side: isize,
}

impl InfiniteGrid {
    pub(crate) fn new(contents: Grid<bool>) -> Self {
        debug_assert_eq!(contents.cols(), contents.rows());
        Self {
            side: contents.cols().try_into().unwrap(),
            contents,
        }
    }

    fn original_coords(&self, (y, x): (isize, isize)) -> (usize, usize) {
        (y.rem_euclid(self.side) as usize, x.rem_euclid(self.side) as usize)
    }

    /// Return the macro-coordinates of the given position.
    fn metatile_coords(&self, (y, x): (isize, isize)) -> (isize, isize) {
        let my = if y >= 0 { y / self.side } else { (y + 1) / self.side - 1 };
        let mx = if x >= 0 { x / self.side } else { (x + 1) / self.side - 1 };
        (my, mx)
    }
}

impl std::ops::Index<(isize, isize)> for InfiniteGrid {
    type Output = bool;

    fn index(&self, (y, x): (isize, isize)) -> &Self::Output {
        &self.contents[self.original_coords((y, x))]
    }
}

fn dijkstra(
    map: &InfiniteGrid,
    start_pos: (usize, usize),
    total_steps: usize,
    mut mark_reachable: impl FnMut((isize, isize)),
) {
    let mut q = BucketQueue::<Vec<_>>::new();
    q.push((start_pos.0 as isize, start_pos.1 as isize), 0);

    let mut dist = HashMap::default();
    dist.insert((start_pos.0 as isize, start_pos.1 as isize), 0);

    let mut visited = HashSet::default();

    while let Some(steps_taken) = q.min_priority() {
        let (x, y) = q.pop(steps_taken).unwrap();
        visited.insert((x, y));

        if steps_taken % 2 == total_steps % 2 {
            mark_reachable((x, y));
        }

        if steps_taken == total_steps {
            continue;
        }

        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        neighbors
            .into_iter()
            .filter(|&(x, y)| !map[(y, x)])
            .filter(|&(x, y)| !visited.contains(&(x, y)))
            .for_each(|next| {
                let alt = steps_taken + 1;
                if dist.get(&next).map_or(true, |&d| d > alt) {
                    dist.insert(next, alt);
                    q.push(next, alt);
                }
            });
    }
}

pub fn solve(map: &InfiniteGrid, start_pos: (usize, usize), total_steps: usize) -> usize {
    let modulo = total_steps as isize % map.side;

    let base = 3 * map.side + modulo;

    let mut reachable_by_metatile = HashMap::<_, usize>::default();
    dijkstra(map, start_pos, base as _, |(x, y)| {
        let (my, mx) = map.metatile_coords((y, x));
        *reachable_by_metatile.entry((my, mx)).or_default() += 1;
    });

    let mut freq = HashMap::<_, usize>::default();
    for &v in reachable_by_metatile.values() {
        *freq.entry(v).or_default() += 1;
    }

    let n = total_steps / map.side as usize;

    freq.into_iter()
        .map(|(k, v)| {
            k * match v {
                3 => n,
                2 => n - 1,
                1 => 1,
                4 => (n - 1) * (n - 1),
                9 => n * n,
                _ => unreachable!(),
            }
        })
        .sum()
}
