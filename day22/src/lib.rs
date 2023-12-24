use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};

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

    loop {
        let mut fallen = false;
        simulate_step(&mut bricks, |_| fallen = true);
        if !fallen {
            break;
        }
    }

    let mut supports = HashMap::new();
    for idx in 0..bricks.len() {
        let brick = &bricks[idx];
        if brick.start.z == 0 {
            continue;
        }
        let target_z = brick.start.z - 1;

        let bricks = &bricks;
        let supported_by = (brick.start.x..=brick.end.x).flat_map(|x| {
            (brick.start.y..=brick.end.y).flat_map(move |y| {
                bricks
                    .iter()
                    .copied()
                    .filter(move |support| support.end.z == target_z)
                    .filter(move |support| {
                        support.start.x <= x && support.end.x >= x && support.start.y <= y && support.end.y >= y
                    })
            })
        });
        supports.insert(brick, supported_by.collect::<Vec<_>>());
    }

    let mut p1 = 0;
    let mut p2 = 0;

    bricks
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let mut hypot_bricks = bricks.clone();
            hypot_bricks.remove(idx);

            let mut fallen_as_a_result = HashSet::new();

            loop {
                let prev = fallen_as_a_result.len();
                simulate_step(&mut hypot_bricks, |brick| {
                    fallen_as_a_result.insert(brick.id);
                });
                if fallen_as_a_result.len() == prev {
                    break;
                }
            }

            fallen_as_a_result.len()
        })
        .for_each(|fall_count| {
            if fall_count == 0 {
                p1 += 1;
            } else {
                p2 += fall_count;
            }
        });

    (p1, p2)
}

fn simulate_step(bricks: &mut Vec<Brick>, mut mark_fallen: impl FnMut(&Brick)) {
    for idx in 0..bricks.len() {
        let brick = &bricks[idx];
        if brick.start.z == 0 {
            continue;
        }
        let target_z = brick.start.z - 1;

        let has_support = (brick.start.x..=brick.end.x).any(|x| {
            (brick.start.y..=brick.end.y).any(|y| {
                bricks
                    .iter()
                    .filter(|support| support.end.z == target_z)
                    .any(|support| {
                        support.start.x <= x && support.end.x >= x && support.start.y <= y && support.end.y >= y
                    })
            })
        });
        if !has_support {
            let brick = &mut bricks[idx];
            brick.start.z -= 1;
            brick.end.z -= 1;
            mark_fallen(brick);
        }
    }
}
