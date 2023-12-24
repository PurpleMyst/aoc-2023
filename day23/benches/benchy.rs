use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use day23::part2;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/input.txt")
        .replace('>', ".")
        .replace('<', ".")
        .replace('^', ".")
        .replace('v', ".");

    c.bench_function("load_graph_map", |b| {
        b.iter(|| part2::load_graph_map(black_box(&input)))
    });
    let (graph, start, goal) = part2::load_graph_map(&input);
    c.bench_with_input(
        BenchmarkId::new("convert_to_graph", "input"),
        &(&graph, start, goal),
        |b, &(graph, start, goal)| {
            b.iter(|| part2::convert_to_graph(black_box(graph.clone()), black_box(start), black_box(goal)))
        },
    );
    c.bench_with_input(
        BenchmarkId::new("do_search", "input"),
        &part2::convert_to_graph(graph, start, goal),
        |b, (graph, start, goal)| {
            b.iter(|| part2::do_search(black_box(graph), black_box(*start), black_box(*goal), 0, 0))
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
