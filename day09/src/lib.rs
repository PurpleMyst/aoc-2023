use std::fmt::Display;

fn diff(x: &mut Vec<i64>) {
    let mut first = x.remove(0);
    for value in x.iter_mut() {
        let tmp = *value;
        *value = *value - first;
        first = tmp;
    }
}

fn part1() -> i64 {
    include_str!("input.txt").lines().map(|line| {
        let mut report: Vec<i64> = line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();
        let mut extrapolated_value = *report.last().unwrap();
        while !report.iter().all(|&n| n == 0) {
            diff(&mut report);
            extrapolated_value += report.last().unwrap();
        }
        extrapolated_value
    }).sum()
}

fn part2() -> i64 {
    let mut history = Vec::new();
    include_str!("input.txt").lines().map(|line| {
        let mut report: Vec<i64> = line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();
        while !report.iter().all(|&n| n == 0) {
            history.push(*report.first().unwrap());
            diff(&mut report);
        }
        history.drain(..).rev().fold(0, |acc, v| {
            v - acc
        })
    }).sum()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    rayon::join(|| part1(), || part2())
}
