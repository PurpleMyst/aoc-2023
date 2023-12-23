use ::bucket_queue::{BucketQueue, LastInFirstOutQueue, Queue};
use hibitset::BitSet;

use rustc_hash::FxHashMap as HashMap;

pub(crate) const TOTAL_STEPS: usize = 26_501_365;

pub(crate) struct InfiniteGrid {
    pub(crate) contents: BitSet,
    pub(crate) side: i32,
}

impl InfiniteGrid {
    pub(crate) fn new(contents: BitSet, side: i32) -> Self {
        Self { contents, side }
    }

    fn original_coords(&self, (y, x): (i32, i32)) -> (usize, usize) {
        (y.rem_euclid(self.side) as usize, x.rem_euclid(self.side) as usize)
    }

    /// Return the macro-coordinates of the given position.
    fn metatile_coords(&self, (y, x): (i32, i32)) -> (i32, i32) {
        let my = if y >= 0 { y / self.side } else { (y + 1) / self.side - 1 };
        let mx = if x >= 0 { x / self.side } else { (x + 1) / self.side - 1 };
        (my, mx)
    }

    fn is_wall(&self, (y, x): (i32, i32)) -> bool {
        let (oy, ox) = self.original_coords((y, x));
        self.contents.contains((oy as i32 * self.side + ox as i32) as u32)
    }
}

fn dijkstra(
    map: &InfiniteGrid,
    start_pos: (usize, usize),
    total_steps: usize,
    mut mark_reachable: impl FnMut((i32, i32)),
) {
    let reachable_parity = total_steps % 2;
    let start_pos = (start_pos.0 as i32, start_pos.1 as i32);

    let mut q = BucketQueue::<Vec<_>>::new();
    q.push((start_pos.0, start_pos.1), 0);

    let y_min = start_pos.0 - total_steps as i32;
    let y_max = start_pos.0 + total_steps as i32;
    let x_min = start_pos.1 - total_steps as i32;
    let x_max = start_pos.1 + total_steps as i32;
    let area = (y_max - y_min + 1) * (x_max - x_min + 1);
    let pos2idx = |(x, y)| ((y - y_min) * (x_max - x_min + 1) + (x - x_min)) as usize;

    let mut dist = vec![usize::MAX; area as usize].into_boxed_slice();
    dist[pos2idx(start_pos)] = 0;

    let mut visited = BitSet::with_capacity(area as u32);

    while let Some(steps_taken) = q.min_priority() {
        let (x, y) = q.pop(steps_taken).unwrap();
        let idx = (y - y_min) * (x_max - x_min + 1) + (x - x_min);
        visited.add(idx as u32);

        if steps_taken % 2 == reachable_parity {
            mark_reachable((x, y));
        }

        if steps_taken == total_steps {
            continue;
        }

        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        neighbors
            .into_iter()
            .filter(|&(x, y)| !map.is_wall((y, x)))
            .map(|neighbor| (neighbor, pos2idx(neighbor)))
            .filter(|&(_, idx)| !visited.contains(idx as u32))
            .for_each(|(next_pos, next_idx)| {
                let alt = steps_taken + 1;
                if dist[next_idx] > alt {
                    dist[next_idx] = alt;
                    q.push(next_pos, alt);
                }
            });
    }
}

pub fn solve(map: &InfiniteGrid, start_pos: (usize, usize), total_steps: usize) -> usize {
    let modulo = total_steps as i32 % map.side;

    let base = 3 * map.side + modulo;
    let (my_min, mx_min) = map.metatile_coords((start_pos.0 as i32 - base, start_pos.1 as i32 - base));
    let (my_max, mx_max) = map.metatile_coords((start_pos.0 as i32 + base, start_pos.1 as i32 + base));
    let area = (my_max - my_min + 1) * (mx_max - mx_min + 1);
    let mpos2idx = |(mx, my)| ((my - my_min) * (mx_max - mx_min + 1) + (mx - mx_min)) as usize;

    let mut reachable_by_metatile = vec![0u16; area as usize].into_boxed_slice();
    dijkstra(map, start_pos, base as _, |(x, y)| {
        let (my, mx) = map.metatile_coords((y, x));
        reachable_by_metatile[mpos2idx((mx, my))] += 1;
    });

    let mut freq = HashMap::<_, usize>::default();
    for &v in reachable_by_metatile.iter() {
        if v != 0 {
            *freq.entry(v).or_default() += 1;
        }
    }

    let n = total_steps / map.side as usize;
    freq.into_iter()
        .map(|(k, v)| {
            k as usize
                * match v {
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
