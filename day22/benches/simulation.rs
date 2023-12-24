use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

fn bench(c: &mut Criterion) {
    for (id, input) in [
        ("sample", include_str!("../src/sample_input.txt")),
        ("problem", include_str!("../src/input.txt")),
    ] {
        let mut bricks = input
            .lines()
            .zip(0..)
            .map(|(line, id)| day22::Brick::parse(id, line))
            .collect::<Vec<_>>();
        bricks.sort_unstable_by_key(|brick| brick.start.z);

        c.bench_with_input(
            criterion::BenchmarkId::new("day22_simulate_step", id),
            &bricks,
            |b, bricks| {
                b.iter_batched(
                    || bricks.clone(),
                    |mut bricks| while day22::simulate_step(&mut bricks) {},
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
