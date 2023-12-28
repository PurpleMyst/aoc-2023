use std::fmt::{Debug, Display};

use arrayvec::ArrayVec;
use hibitset::{BitSet, BitSetLike};
use itertools::iproduct;
use rayon::prelude::*;

use rustc_hash::FxHashSet as HashSet;

const SIDE: usize = 10;

type SupportTree = Vec<ArrayVec<usize, 3>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
    pub z: u16,
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
pub struct Brick {
    pub id: usize,
    pub start: Pos,
    pub end: Pos,
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<brick #{}: {:?}..={:?}>", self.id, self.start, self.end)
    }
}

impl Brick {
    pub fn parse(id: usize, s: &str) -> Self {
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
    simulate_until_settled(&mut bricks);

    let supporters = get_support_tree(&bricks);

    bricks.sort_unstable_by_key(|brick| brick.id);

    let mut sole_supporters = HashSet::default();
    for supported_by in &supporters {
        if let [sole_supporter] = supported_by.as_slice() {
            sole_supporters.insert(*sole_supporter);
        }
    }
    let p1 = bricks.len() - sole_supporters.len();

    let mut supportees = vec![ArrayVec::<_, 4>::new(); bricks.len()];
    for (&supportee, supported_by) in bricks.iter().zip(supporters.iter()) {
        for &supporter in supported_by {
            supportees[supporter].push(supportee.id);
        }
    }

    let p2 = sole_supporters
        .into_par_iter()
        .map(|brick| {
            let mut falling = BitSet::new();
            falling.add(brick as u32);

            let mut q = vec![brick];
            while let Some(brick) = q.pop() {
                let children = &supportees[brick];
                for &child in children {
                    if supporters[child]
                        .iter()
                        .all(|&support| falling.contains(support as u32))
                    {
                        falling.add(child as u32);
                        q.push(child);
                    }
                }
            }

            falling.iter().count() - 1
        })
        .sum::<usize>();

    (p1, p2)
}

pub fn simulate_until_settled(bricks: &mut Vec<Brick>) {
    bricks.sort_unstable_by_key(|brick| (brick.start.z, brick.end.z));

    let mut height_map = vec![0; SIDE * SIDE];

    for brick in bricks {
        let fall_to = iproduct!(brick.start.x..=brick.end.x, brick.start.y..=brick.end.y)
            .map(|(x, y)| height_map[x as usize + y as usize * SIDE])
            .max()
            .unwrap();

        brick.end.z = fall_to + (brick.end.z - brick.start.z);
        brick.start.z = fall_to;

        iproduct!(brick.start.x..=brick.end.x, brick.start.y..=brick.end.y)
            .for_each(|(x, y)| height_map[x as usize + y as usize * SIDE] = brick.end.z + 1);
    }
}

fn intersects_xy(a: &Brick, b: &Brick) -> bool {
    a.start.x <= b.end.x && a.end.x >= b.start.x && a.start.y <= b.end.y && a.end.y >= b.start.y
}

fn get_support_tree(bricks: &[Brick]) -> SupportTree {
    let mut tree = vec![ArrayVec::new(); bricks.len()];
    (0..bricks.len()).for_each(|idx| {
        let brick = &bricks[idx];

        if brick.start.z == 0 {
            return;
        }

        let target_z = brick.start.z - 1;

        tree[brick.id] = bricks
            .iter()
            .take(idx)
            .filter(|support| support.end.z == target_z)
            .filter(|support| intersects_xy(brick, support))
            .map(|support| support.id)
            .collect();
    });
    tree
}
