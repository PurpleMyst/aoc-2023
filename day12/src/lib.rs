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
    let mut unfolded_springs = String::with_capacity(6 * springs.len());
    let mut unfolded_config = String::with_capacity(6 * config.len());
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

    // Calculate the suffix sums of the groups, as we'll need them later to prune invalid states.
    let mut groups_suffix_sums: Vec<usize> = groups
        .iter()
        .rev()
        .scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        })
        .collect();
    groups_suffix_sums.reverse();
    groups_suffix_sums.push(0);

    // Double-buffered HashMaps to store the states, counting repetitions.
    let mut states: HashMap<(usize, usize), usize> = HashMap::default();
    states.insert((0, 0), 1);
    let mut new_states = HashMap::default();

    for (&spring, springs_left) in springs.iter().zip((0..springs.len()).rev()) {
        // Iterate over the states, advancing them.
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
                    // Process as '#'.
                    if group_idx < groups.len() && group_len < groups[group_idx] {
                        *new_states.entry((group_idx, group_len + 1)).or_default() += permutations;
                    }

                    // Process as '.'.
                    if group_len == 0 {
                        *new_states.entry((group_idx, group_len)).or_default() += permutations;
                    } else if group_len == groups[group_idx] {
                        *new_states.entry((group_idx + 1, 0)).or_default() += permutations;
                    }
                }
                _ => unreachable!(),
            }
        }

        // Swap over the new states, keeping in mind we've already cleared the old states via drain.
        std::mem::swap(&mut states, &mut new_states);

        // Prune the states that are guaranteed to be invalid.
        states.retain(|&(group_idx, group_len), _| {
            springs_left + group_len >= groups_suffix_sums[group_idx]
        });
    }

    // Return how many states got to the end, staying valid.
    states.values().copied().sum()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    rayon::join(|| part1(include_str!("input.txt")), || part2(include_str!("input.txt")))
}
