use std::fmt::Display;

use nalgebra::{Matrix4, RowVector4, Vector4};

struct Hailstone {
    position: [f64; 3],
    velocity: [f64; 3],
}

impl std::fmt::Debug for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:2}, {:2}, {:2} @ {:2}, {:2}, {:2}",
            self.position[0], self.position[1], self.position[2], self.velocity[0], self.velocity[1], self.velocity[2]
        )
    }
}

impl Hailstone {
    fn parse(s: &str) -> Self {
        let (p, v) = s.split_once(" @ ").unwrap();
        let mut p = p.split(", ").map(|n| n.trim().parse().unwrap());
        let mut v = v.split(", ").map(|n| n.trim().parse().unwrap());
        Self {
            position: [p.next().unwrap(), p.next().unwrap(), p.next().unwrap()],
            velocity: [v.next().unwrap(), v.next().unwrap(), v.next().unwrap()],
        }
    }

    fn time_for(&self, coord: usize, target: f64) -> f64 {
        (target - self.position[coord]) / self.velocity[coord]
    }

    fn trajectory(&self) -> (f64, f64) {
        let q1 = self.position[1] - self.position[0] * self.velocity[1] / self.velocity[0];
        let m1 = self.velocity[1] / self.velocity[0];
        (q1, m1)
    }

    fn intersection_xy(&self, other: &Self) -> (f64, f64) {
        let (q1, m1) = self.trajectory();
        let (q2, m2) = other.trajectory();
        let x = (q2 - q1) / (m1 - m2);
        let y = m1 * x + q1;
        (x, y)
    }
}

fn in_test_area(coord: f64) -> bool {
    (200000000000000. ..=400000000000000.).contains(&coord)
}

fn do_part1(hailstones: &Vec<Hailstone>) -> i32 {
    let mut part1 = 0;
    for (i, h1) in hailstones.iter().enumerate() {
        for h2 in hailstones.iter().skip(i + 1) {
            let (x, y) = h1.intersection_xy(h2);
            if h1.time_for(0, x) < 0. || h2.time_for(0, x) < 0. || h1.time_for(1, y) < 0. || h2.time_for(1, y) < 0. {
                continue;
            }

            if in_test_area(x) && in_test_area(y) {
                part1 += 1;
            }
        }
    }
    part1
}

// adapted from https://www.reddit.com/r/adventofcode/comments/18pnycy/2023_day_24_solutions/kf068o2/
fn to_xy_equations(head: &Hailstone, tail: &[Hailstone]) -> (Matrix4<f64>, Vector4<f64>) {
    let mut m = Matrix4::zeros();
    let mut r = Vector4::zeros();
    for (i, h) in tail.iter().enumerate() {
        let (e, w) = to_xy_equation(head, h);
        m.set_row(i, &e);
        r[i] = w;
    }
    (m, r)
}

fn to_xy_equation(a: &Hailstone, b: &Hailstone) -> (RowVector4<f64>, f64) {
    let (x0, y0, _) = (a.position[0], a.position[1], a.position[2]);
    let (x1, y1, _) = (b.position[0], b.position[1], b.position[2]);
    let (vx0, vy0, _) = (a.velocity[0], a.velocity[1], a.velocity[2]);
    let (vx1, vy1, _) = (b.velocity[0], b.velocity[1], b.velocity[2]);
    let e = RowVector4::new(vy0 - vy1, y1 - y0, vx1 - vx0, x0 - x1);
    let w = vy0 * x0 - vx0 * y0 + vx1 * y1 - vy1 * x1;
    (e, w)
}

fn find_z_dz(x: f64, dx: f64, h0: &Hailstone, h1: &Hailstone) -> (f64, f64) {
    let t0 = (h0.position[0] - x) / (dx - h0.velocity[0]);
    let t1 = (h1.position[0] - x) / (dx - h1.velocity[0]);
    let r0 = h0.position[2] + h0.velocity[2] * t0;
    let r1 = h1.position[2] + h1.velocity[2] * t1;
    let dz = (r1 - r0) / (t1 - t0);
    let z = r0 - dz * t0;
    (z, dz)
}

fn do_part2(hs: &[Hailstone; 5]) -> f64 {
    let (head, tail) = hs.split_first().unwrap();
    let sol = {
        let (m, r) = to_xy_equations(head, tail);
        m.lu().solve(&r).unwrap()
    };
    let x = sol[0];
    let dx = sol[1];
    let y = sol[2];
    let (z, _) = find_z_dz(x, dx, head, &tail[0]);
    x + y + z
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");
    let hailstones = input.lines().map(Hailstone::parse).collect::<Vec<_>>();
    (do_part1(&hailstones), do_part2(hailstones[..5].try_into().unwrap()))
}
