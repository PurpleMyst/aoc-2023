use std::fmt::Display;

fn diff(x: &mut Vec<i64>) {
    let mut first = x.remove(0);
    for value in x.iter_mut() {
        let tmp = *value;
        *value = *value - first;
        first = tmp;
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut report: Vec<i64> = line.split(' ').map(|n| n.parse().unwrap()).collect();
            let mut p1 = *report.last().unwrap();
            let mut p2 = *report.first().unwrap();
            let mut negated = true;
            while !report.iter().all(|&n| n == 0) {
                diff(&mut report);
                let f = *report.first().unwrap();
                p1 += report.last().unwrap();
                p2 += if negated { -f } else { f };
                negated = !negated;
            }
            (p1, p2)
        })
        .reduce(|(p11, p21), (p12, p22)| (p11 + p12, p21 + p22))
        .unwrap()
}
