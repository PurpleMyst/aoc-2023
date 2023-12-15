use std::fmt::Display;

use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;

// Adapted from https://redd.it/18gomx5 with the help of ChatGPT and GitHub Copilot!

fn part1(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(create_permutations)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(unfold)
        .map(|line| create_permutations(&line))
        .sum()
}

fn unfold(line: &str) -> String {
    let (springs, config) = line.split_once(' ').unwrap();
    let mut unfolded_springs = String::new();
    let mut unfolded_config = String::new();
    for i in 0..5 {
        if i != 0 {
            unfolded_springs.push('?');
            unfolded_config.push(',');
        }
        unfolded_springs.push_str(springs);
        unfolded_config.push_str(config);
    }
    format!("{} {}", unfolded_springs, unfolded_config)
}

fn create_permutations(line: &str) -> usize {
    // Parse the input.
    let (springs, groups) = line.split_once(' ').unwrap();
    let groups: Vec<usize> = groups.split(',').map(|s| s.parse().unwrap()).collect();
    let springs: Vec<char> = springs.chars().collect();

    let mut states: HashMap<(usize, usize), usize> = HashMap::default();
    states.insert((0, 0), 1);
    let mut new_states = HashMap::default();

    for (&spring, springs_left) in springs.iter().zip((0..springs.len()).rev()) {
        for ((group_idx, group_len), permutations) in states.drain() {
            match spring {
                '#' => {
                    // If we haven't run out of groups and the current group is not full, we can
                    // advance this state.
                    if group_idx < groups.len() && group_len < groups[group_idx] {
                        *new_states.entry((group_idx, group_len + 1)).or_default() += permutations;
                    }
                }
                '.' => {
                    if group_len == 0 {
                        // We're not in a group, so leave the state as-is.
                        *new_states.entry((group_idx, group_len)).or_default() += permutations;
                    } else if group_len == groups[group_idx] {
                        // The current group has been broken. Advance to the next group.
                        *new_states.entry((group_idx + 1, 0)).or_default() += permutations;
                    }
                }
                '?' => {
                    // Process as #
                    if group_idx < groups.len() && group_len < groups[group_idx] {
                        // new_permutations.push((group_idx, group_len + 1, permutations));
                        *new_states.entry((group_idx, group_len + 1)).or_default() += permutations;
                    }

                    // Process as .
                    if group_len == 0 {
                        *new_states.entry((group_idx, group_len)).or_default() += permutations;
                    } else if group_len == groups[group_idx] {
                        *new_states.entry((group_idx + 1, 0)).or_default() += permutations;
                    }
                }
                _ => unreachable!(),
            }
        }

        std::mem::swap(&mut states, &mut new_states);
        states.retain(|&(group_idx, group_len), _| {
            if group_idx >= groups.len() {
                group_len == 0
            } else {
                springs_left + group_len >= groups.iter().skip(group_idx).sum()
            }
        });
    }

    states.values().copied().sum()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    rayon::join(|| part1(include_str!("input.txt")), || part2(include_str!("input.txt")))
}
