use criterion::{criterion_group, criterion_main, Criterion};

use day21::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day20");
    let (map, start_pos) = load_input(include_str!("../src/sample_input.txt"));

    for example in [6, 10, 50, 100, 500, 1000] {
        group.throughput(criterion::Throughput::Elements(example as u64));
        group.bench_with_input(
            format!("{example} steps"),
            &(&map, start_pos, example),
            |b, (map, start_pos, example)| {
                b.iter(|| {
         do_solve_part2(map, *start_pos, *example)
                })
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
