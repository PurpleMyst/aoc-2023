use std::fmt::Display;

fn part1() -> impl Display {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    let mut seeds = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let _ = lines.next(); // burn empty line

    let mut next_seeds: Vec<usize> = Vec::new();
    while let Some(_) = lines.next() {
        // while there is a next map
        for next_line in lines.by_ref() {
            if next_line.is_empty() {
                break;
            }

            let mut it = next_line.split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap());
            let dest_start = it.next().unwrap();
            let src_start = it.next().unwrap();
            let range_len = it.next().unwrap();

            seeds.retain(|&seed| {
                if (src_start..src_start + range_len).contains(&seed) {
                    next_seeds.push(seed - src_start + dest_start);
                    false
                } else {
                    true
                }
            });
        }
        seeds.append(&mut next_seeds);
    }

    seeds.into_iter().min().unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SeedRange {
    start: usize,
    len: usize,
}

const EMPTY_RANGE: SeedRange = SeedRange { start: 0, len: 0 };

impl SeedRange {
    fn intersection(&self, needle: &Self) -> (Self, Self, Self) {
        let self_end = self.start + self.len;
        let needle_end = needle.start + needle.len;

        if self_end < needle.start || needle_end < self.start {
            return (*self, EMPTY_RANGE, EMPTY_RANGE);
        }

        let common_start = self.start.max(needle.start);
        let common_end = self_end.min(needle_end);
        let common = SeedRange {
            start: common_start,
            len: common_end - common_start,
        };
        let left_start = self.start;
        let left_end = common_start;
        let left = SeedRange {
            start: left_start,
            len: left_end - left_start,
        };
        let right_start = common_end;
        let right_end = self_end;
        let right = SeedRange {
            start: right_start,
            len: right_end - right_start,
        };
        (left, common, right)
    }
}

impl Display for SeedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..{}]", self.start, self.start + self.len)
    }
}

fn part2() -> impl Display {
    let input = include_str!("input.txt");
    let mut lines = input.lines();

    let mut seeds = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|chunk| SeedRange {
            start: chunk[0],
            len: chunk[1],
        })
        .collect::<Vec<_>>();
    let _ = lines.next(); // burn empty line

    let mut next_seeds: Vec<SeedRange> = Vec::new();
    let mut add_buffer = Vec::new();
    while let Some(_header) = lines.next() {
        // while there is a next map
        for next_line in lines.by_ref() {
            if next_line.is_empty() {
                break;
            }

            let mut it = next_line.split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap());
            let dest_start = it.next().unwrap();
            let src_start = it.next().unwrap();
            let range_len = it.next().unwrap();

            let dest_offset = dest_start as isize - src_start as isize;

            let src_range = SeedRange {
                start: src_start,
                len: range_len,
            };

            seeds.retain_mut(|seed| {
                let (left, common, right) = seed.intersection(&src_range);
                *seed = left;
                if right.len != 0 {
                    add_buffer.push(right);
                }
                if common.len != 0 {
                    let new_seed = SeedRange {
                        start: common.start.checked_add_signed(dest_offset).unwrap(),
                        len: common.len,
                    };
                    next_seeds.push(new_seed);
                }
                left.len != 0
            });
            seeds.append(&mut add_buffer);
        }
        seeds.append(&mut next_seeds);
    }

    seeds.into_iter().map(|sr| sr.start).min().unwrap()
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    (part1(), part2())
}
