use std::fmt::Display;

fn calculate_answer((time, distance): (f64, f64)) -> f64 {
    // assuming I hold the button for v seconds, I'll then travel v * time_remaining.
    // let's call t the max time alloted and d the distance to travel, we want
    // v * (t - v) >= d
    // v * t - v^2 >= d
    // v^2 - v * t + d <= 0
    // which is a quadratic equation in v. we can build the parabola
    // and find the integer range in which this works.

    let v2 = (time + (time * time - 4. * distance).sqrt()) / 2.;
    let v1 = (time - (time * time - 4. * distance).sqrt()) / 2.;

    let low = if v1 == v1.ceil() { v1 + 1. } else { v1.ceil() };
    let high = if v2 == v2.floor() { v2 } else { v2.ceil() };

    high - low
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut input = include_str!("input.txt").lines();
    let times = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<f64>().unwrap());
    let distances = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<f64>().unwrap());
    let part1 = times.zip(distances).map(calculate_answer).product::<f64>();

    let mut input = include_str!("input.txt").lines();
    let time = input
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<f64>()
        .unwrap();
    let distance = input
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<f64>()
        .unwrap();
    let part2 = calculate_answer((time, distance));

    (part1, part2)
}
