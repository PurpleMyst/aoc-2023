use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parts(c: &mut Criterion) {
    c.bench_function("part1", |b| {
        b.iter(|| day23::part1::solve(black_box(include_str!("../src/input.txt"))));
    });

    c.bench_with_input(
        criterion::BenchmarkId::new("part2", "input.txt"),
        &include_str!("../src/input.txt").replace(['>', '<', '^', 'v'], "."),
        |b, i| {
            b.iter(|| day23::part2::solve(black_box(i)));
        },
    );
}

criterion_group!(benches, parts);
criterion_main!(benches);
