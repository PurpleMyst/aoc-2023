use std::fmt::{Debug, Display};

use grid::Grid;
use hibitset::{BitSet, BitSetLike};
use itertools::Itertools;
use petgraph::prelude::*;
use rayon::prelude::*;

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: u64,
    y: u64,
    z: u64,
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pos")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl Pos {
    fn parse(s: &str) -> Self {
        let mut it = s.split(',').map(|s| s.parse().unwrap());
        Self {
            x: it.next().unwrap(),
            y: it.next().unwrap(),
            z: it.next().unwrap(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Brick {
    id: usize,
    start: Pos,
    end: Pos,
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<brick #{}: {:?}..={:?}>", self.id, self.start, self.end)
    }
}

impl Brick {
    fn parse(id: usize, s: &str) -> Self {
        let (start, end) = s.split_once('~').unwrap();
        let start = Pos::parse(start);
        let end = Pos::parse(end);
        let (start, end) = if start.x > end.x || start.y > end.y || start.z > end.z {
            (end, start)
        } else {
            (start, end)
        };
        Self { id, start, end }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut bricks = include_str!("input.txt")
        .lines()
        .zip(0..)
        .map(|(line, id)| Brick::parse(id, line))
        .collect::<Vec<_>>();

    // Let the bricks hit the floor.
    loop {
        let mut fallen = false;
        simulate_step(&mut bricks, |_| fallen = true);
        if !fallen {
            break;
        }
    }

    let supporters = get_support_tree(&bricks);

    let mut sole_supporters = HashSet::default();
    for (_brick, supported_by) in &supporters {
        if let [sole_supporter] = supported_by.as_slice() {
            sole_supporters.insert(*sole_supporter);
        }
    }
    let p1 = bricks.len() - sole_supporters.len();

    let mut supportees = HashMap::default();
    for (&supportee, supported_by) in &supporters {
        for &supporter in supported_by {
            supportees.entry(supporter).or_insert_with(Vec::new).push(supportee);
        }
    }

    let p2 = sole_supporters
        .into_par_iter()
        .map(|brick| {
            let mut falling = BitSet::new();
            falling.add(brick.id as u32);

            let mut q = vec![brick];
            while let Some(brick) = q.pop() {
                let Some(children) = supportees.get(&brick) else {
                    continue;
                };
                for child in children {
                    if supporters[child]
                        .iter()
                        .all(|support| falling.contains(support.id as u32))
                    {
                        falling.add(child.id as u32);
                        q.push(child);
                    }
                }
            }

            falling.iter().count() - 1
        })
        .sum::<usize>();

    (p1, p2)
}

fn intersects_xy(a: &Brick, b: &Brick) -> bool {
    a.start.x <= b.end.x && a.end.x >= b.start.x && a.start.y <= b.end.y && a.end.y >= b.start.y
}

type SupportTree<'a> = HashMap<&'a Brick, Vec<&'a Brick>>;

fn get_support_tree(bricks: &[Brick]) -> SupportTree {
    // Sort bricks by their bottom. Maybe they're already sorted.

    (0..bricks.len())
        .map(|idx| {
            let brick = &bricks[idx];

            if brick.start.z == 0 {
                return (brick, vec![]);
            }

            let target_z = brick.start.z - 1;

            (
                brick,
                bricks
                    .iter()
                    .take(idx)
                    .filter(|support| support.end.z == target_z)
                    .filter(|support| intersects_xy(brick, support))
                    .collect(),
            )
        })
        .collect()
}

fn simulate_step(bricks: &mut [Brick], mut mark_fallen: impl FnMut(&Brick)) {
    // Sort bricks by their bottom.
    bricks.sort_by_key(|brick| brick.start.z);

    for idx in 0..bricks.len() {
        let brick = &bricks[idx];

        if brick.start.z == 0 {
            continue;
        }

        let target_z = brick.start.z - 1;

        let has_support = bricks
            .iter()
            .take(idx)
            .filter(|support| support.end.z == target_z)
            .any(|support| intersects_xy(brick, support));

        if !has_support {
            let brick = &mut bricks[idx];
            brick.start.z -= 1;
            brick.end.z -= 1;
            mark_fallen(brick);
        }
    }
}
