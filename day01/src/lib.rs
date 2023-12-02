use std::fmt::Display;

const DIGITS: [&[u8]; 9] = [b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine"];

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let part1 = input
        .lines()
        .map(|line| {
            let mut it = line.bytes().filter(|ch| ch.is_ascii_digit());
            let first = it.clone().next().unwrap();
            let second = it.next_back().unwrap();
            usize::from(10 * (first - b'0') + (second - b'0'))
        })
        .sum::<usize>();

    let part2 = input
        .lines()
        .map(|line| {
            let mut line = line.as_bytes();
            let mut first = None;
            let mut last = None;
            while !line.is_empty() {
                let digit = if line[0].is_ascii_digit() {
                    Some(usize::from(line[0] - b'0'))
                } else if let Some(n) = DIGITS.iter().position(|&d| line.starts_with(d)) {
                    Some(n + 1)
                } else {
                    None
                };
                first = first.or(digit);
                last = digit.or(last);
                line = &line[1..];
            }
            let answer = first.unwrap() * 10 + last.unwrap();
            answer
        })
        .sum::<usize>();

    (part1, part2)
}
