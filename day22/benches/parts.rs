use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use day22::*;

fn parts(c: &mut Criterion) {
    c.bench_function("simulate_untiL_settled", |b| {
        b.iter_batched(
            || {
                black_box(
                    include_str!("../src/input.txt")
                        .lines()
                        .zip(0..)
                        .map(|(line, id)| Brick::parse(id, line))
                        .collect::<Vec<_>>(),
                )
            },
            |mut bricks| simulate_until_settled(&mut bricks),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, parts);
criterion_main!(benches);
