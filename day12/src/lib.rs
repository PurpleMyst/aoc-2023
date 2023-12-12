use std::{collections::HashMap, fmt::Display};

use rayon::prelude::*;

const WORKING: u8 = b'.';
const BROKEN: u8 = b'#';
const UNKNOWN: u8 = b'?';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    records: &'static [u8],
    current_group: Option<u8>,
    remaining_groups: &'static [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RecordResult {
    Die,
    Continue(State),
    Branch(State, State),
}

fn process_record(current_record: u8, remaining_records: &'static [u8], state: State) -> RecordResult {
    match current_record {
        WORKING => {
            // We've found a working one!
            match state.current_group {
                // If there were no more broken springs remaining in our current group,
                // this is what we should expect: the state should continue to live.
                // This is also what should happen if we weren't even in a group
                Some(0) | None => RecordResult::Continue(State {
                    records: remaining_records,
                    current_group: None,
                    remaining_groups: state.remaining_groups,
                }),

                // If we were expecting more broken springs and we found a working one, this state
                // should die as it is based on an invalid hypothesis.
                Some(..) => RecordResult::Die,
            }
        }

        BROKEN => {
            // We've found a broken one!
            match state.current_group {
                // If we were expecting no more broken ones, this state should die as it is
                // based on an invalid hypothesis.
                Some(0) => RecordResult::Die,

                // If we weren't in a group, let's start one.
                None => {
                    if let Some((&current_group, remaining_groups)) = state.remaining_groups.split_first() {
                        RecordResult::Continue(State {
                            records: remaining_records,
                            current_group: Some(current_group - 1),
                            remaining_groups,
                        })
                    } else {
                        RecordResult::Die
                    }
                }

                // At this point, we're in a group where we were expecting at least one more
                // broken spring. Let's update the state!
                Some(n) => RecordResult::Continue(State {
                    records: remaining_records,
                    current_group: Some(n - 1),
                    remaining_groups: state.remaining_groups,
                }),
            }
        }

        UNKNOWN => {
            match (
                process_record(WORKING, remaining_records, state.clone()),
                process_record(BROKEN, remaining_records, state),
            ) {
                (RecordResult::Die, RecordResult::Die) => RecordResult::Die,
                (RecordResult::Die, r @ RecordResult::Continue(..)) => r,
                (r @ RecordResult::Continue(..), RecordResult::Die) => r,
                (RecordResult::Continue(s1), RecordResult::Continue(s2)) => RecordResult::Branch(s1, s2),
                _ => unreachable!(),
            }
        }

        _ => unreachable!(),
    }
}

#[michie::memoized(key_expr = state, store_type = HashMap<State, u64>)]
fn process_state(state: State) -> u64 {
    let Some((&current_record, remaining_records)) = state.records.split_first() else {
        let is_valid = matches!(state.current_group, Some(0) | None) && state.remaining_groups.is_empty();
        return if is_valid { 1 } else { 0 };
    };
    match process_record(current_record, remaining_records, state) {
        RecordResult::Die => 0,
        RecordResult::Continue(s) => process_state(s),
        RecordResult::Branch(s1, s2) => process_state(s1) + process_state(s2),
    }
}

fn process_row(records: &'static [u8], groups: &'static [u8]) -> u64 {
    return process_state(State {
        records,
        current_group: None,
        remaining_groups: groups,
    });
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt").lines().map(|line| {
        let (records, groups) = line.split_once(' ').unwrap();
        let records = records.as_bytes();
        let groups = groups
            .split(',')
            .map(|group| group.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        (records, groups)
    });

    let part1 = input
        .clone()
        .map(|(records, groups)| process_row(records, groups.leak()))
        .sum::<u64>();

    let part2 = input
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(records, groups)| {
            let mut new_records = Vec::new();
            for i in 0..5 {
                if i != 0 {
                    new_records.push(UNKNOWN);
                }
                new_records.extend_from_slice(records);
            }
            let mut new_groups = Vec::new();
            for _ in 0..5 {
                new_groups.extend_from_slice(&groups);
            }
            process_row(new_records.leak(), new_groups.leak())
        })
        .sum::<u64>();

    (part1, part2)
}
