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
    let (all_springs, spring_condition) = line.split_once(' ').unwrap();

    let conditions: Vec<usize> = spring_condition.split(',').map(|s| s.parse().unwrap()).collect();
    let springs: Vec<char> = all_springs.chars().collect();

    let mut spring_permutations = vec![(0, 0, 1)];
    let mut spring_permutation_counts: HashMap<(usize, usize), usize> = HashMap::default();
    spring_permutation_counts.insert((0, 0), 1);

    let mut springs_checked = 0;
    for &spring in springs.iter() {
        let mut new_permutations = vec![];
        for (&(group_idx, group_len), &permutations) in spring_permutation_counts.iter() {
            match spring {
                '#' => {
                    // If we haven't run out of groups and the current group is not full, we can
                    // advance this state.
                    if group_idx < conditions.len() && group_len < conditions[group_idx] {
                        new_permutations.push((group_idx, group_len + 1, permutations));
                    }
                }
                '?' => {
                    // Process as #
                    if group_idx < conditions.len() && group_len < conditions[group_idx] {
                        new_permutations.push((group_idx, group_len + 1, permutations));
                    }

                    // Process as .
                    if group_len == 0 {
                        new_permutations.push((group_idx, group_len, permutations));
                    } else if group_len == conditions[group_idx] {
                        new_permutations.push((group_idx + 1, 0, permutations));
                    }
                }
                '.' => {
                    if group_len == 0 {
                        // We're not in a group, so leave the state as-is.
                        new_permutations.push((group_idx, group_len, permutations));
                    } else if group_len == conditions[group_idx] {
                        // The current group has been broken. Advance to the next group.
                        new_permutations.push((group_idx + 1, 0, permutations));
                    }
                }
                _ => unreachable!(),
            }
        }

        spring_permutations = new_permutations;
        springs_checked += 1;

        let springs_left = all_springs.len() - springs_checked;
        spring_permutations = spring_permutations
            .into_iter()
            .filter(|&(group_idx, group_len, _)| {
                if group_idx > conditions.len() {
                    group_len == 0
                } else {
                    springs_left + group_len >= conditions.iter().skip(group_idx).sum()
                }
            })
            .collect();

        spring_permutation_counts.clear();
        for &(group, amount, permutations) in &spring_permutations {
            *spring_permutation_counts.entry((group, amount)).or_insert(0) += permutations;
        }
    }

    spring_permutations
        .into_iter()
        .map(|(_, _, permutations)| permutations)
        .sum()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    rayon::join(|| part1(include_str!("input.txt")), || part2(include_str!("input.txt")))
}
