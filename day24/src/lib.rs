use std::fmt::Display;

use z3::ast::{Ast, Int, Real};

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

    fn intersection_xy(&self, other: &Self) -> Option<(f64, f64)> {
        let (q1, m1) = self.trajectory();
        let (q2, m2) = other.trajectory();
        let x = (q2 - q1) / (m1 - m2);
        let y = m1 * x + q1;
        Some((x, y))
    }
}

fn in_test_area(coord: f64) -> bool {
    coord >= 200000000000000. && coord <= 400000000000000.
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let hailstones = input.lines().map(Hailstone::parse).collect::<Vec<_>>();

    let mut part1 = 0;
    for (i, h1) in hailstones.iter().enumerate() {
        for h2 in hailstones.iter().skip(i + 1) {
            if let Some((x, y)) = h1.intersection_xy(h2) {
                if h1.time_for(0, x) < 0. || h2.time_for(0, x) < 0. || h1.time_for(1, y) < 0. || h2.time_for(1, y) < 0.
                {
                    continue;
                }

                if in_test_area(x) && in_test_area(y) {
                    part1 += 1;
                }
            }
        }
    }

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);

    let rx = Real::new_const(&ctx, "rx");
    let ry = Real::new_const(&ctx, "ry");
    let rz = Real::new_const(&ctx, "rz");
    let wx = Real::new_const(&ctx, "wx");
    let wy = Real::new_const(&ctx, "wy");
    let wz = Real::new_const(&ctx, "wz");

    let solver = z3::Solver::new(&ctx);

    for (i, h) in hailstones.into_iter().enumerate().take(3) {
        let t_n = Real::new_const(&ctx, format!("t_{}", 1 + i).as_str());
        solver.assert(&(&rx + &wx * &t_n)._eq(
            &(Real::from_int(&Int::from_i64(&ctx, h.position[0] as i64))
                + Real::from_int(&Int::from_i64(&ctx, h.velocity[0] as i64)) * &t_n),
        ));
        solver.assert(&(&ry + &wy * &t_n)._eq(
            &(Real::from_int(&Int::from_i64(&ctx, h.position[1] as i64))
                + Real::from_int(&Int::from_i64(&ctx, h.velocity[1] as i64)) * &t_n),
        ));
        solver.assert(&(&rz + &wz * &t_n)._eq(
            &(Real::from_int(&Int::from_i64(&ctx, h.position[2] as i64))
                + Real::from_int(&Int::from_i64(&ctx, h.velocity[2] as i64)) * &t_n),
        ));
    }
    assert_eq!(solver.check(), z3::SatResult::Sat);
    let model = solver.get_model().unwrap();
    let part2 = model.eval(&rx, true).unwrap().as_real().unwrap().0
        + model.eval(&ry, true).unwrap().as_real().unwrap().0
        + model.eval(&rz, true).unwrap().as_real().unwrap().0;

    (part1, part2)
}
