use std::fmt::Display;

use ahash::{HashMap, HashMapExt};
use rayon::prelude::*;

const START: &str = "AAA";
const GOAL: &str = "ZZZ";

#[derive(Debug)]
struct Branch {
    left: &'static str,
    right: &'static str,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => f.pad("L"),
            Direction::Right => f.pad("R"),
        }
    }
}

fn part1(map: &HashMap<&'static str, Branch>, mut directions_to_take: impl Iterator<Item = Direction>) -> usize {
    let mut current = START;
    for n in 0.. {
        if current == GOAL {
            return n;
        }

        let branch = map.get(current).unwrap();
        current = match directions_to_take.next().unwrap() {
            Direction::Left => branch.left,
            Direction::Right => branch.right,
        };
    }

    unreachable!();
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a > b {
        (a / gcd(a, b)) * b
    } else {
        (b / gcd(a, b)) * a
    }
}

fn part2<I>(map: &HashMap<&'static str, Branch>, directions_to_take: I) -> usize
where
    I: Iterator<Item = Direction> + Clone + Sync,
{
    let starting_nodes = map
        .keys()
        .filter(|&&node| node.ends_with('A'))
        .copied()
        .collect::<Vec<_>>();

    // Each node is independent from the others, so we can figure out the number of steps to reach
    // the goal for each node, and then find the least common multiple of the cycle length of each.
    // We figure this out by AoC experience and by looking at the map: from the starting node, once
    // we reach a Z node that Z node is then part of a cycle that eventually leads back to itself.
    //
    // The input seems to then be designed so that the cycle length is the same as the amount of
    // steps it takes you to reach the goal the first time!
    let cycle_lengths = starting_nodes.into_par_iter().map(|mut node| {
        let mut directions_to_take = directions_to_take.clone();
        let mut steps = 0;
        while !node.ends_with('Z') {
            steps += 1;
            let direction = directions_to_take.next().unwrap();
            let branch = map.get(node).unwrap();
            node = match direction {
                Direction::Left => branch.left,
                Direction::Right => branch.right,
            };
        }
        steps
    });

    // Find the least common multiple of the cycle lengths.
    cycle_lengths.reduce(|| 1, lcm)
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut map = HashMap::new();
    let mut input = include_str!("input.txt").lines();
    let directions_to_take = input
        .next()
        .unwrap()
        .chars()
        .map(|ch| match ch {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .cycle();
    let _ = input.next();
    for line in input {
        let (node, branches) = line.split_once(" = ").unwrap();
        let (left, right) = branches.trim_matches(&['(', ')'][..]).split_once(", ").unwrap();
        map.insert(node, Branch { left, right });
    }

    rayon::join(
        || part1(&map, directions_to_take.clone()),
        || part2(&map, directions_to_take.clone()),
    )
}
