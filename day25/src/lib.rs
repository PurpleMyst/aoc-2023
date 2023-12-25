use std::fmt::Display;

use rand::prelude::*;
use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;

use petgraph::prelude::*;

const SAMPLE_POINTS: usize = 400;

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let graph = load_input();
    let answer = do_solve(graph);
    (answer, "Merry Christmas!")
}

fn load_input() -> Graph<(), (), Undirected, usize> {
    let mut graph = UnGraphMap::<&'static str, ()>::new();
    for line in include_str!("input.txt").lines() {
        let (source, destinations) = line.split_once(": ").unwrap();
        for destination in destinations.split(' ') {
            graph.add_edge(source, destination, ());
        }
    }
    graph.into_graph::<usize>().map(|_, _| (), |_, _| ())
}

// adapted from https://www.reddit.com/r/adventofcode/comments/18qe8qo/2023_day_25_part_1_why_work_hard/keudy5q/
fn do_solve(mut graph: Graph<(), (), Undirected, usize>) -> usize {
    let frequencies = (0..SAMPLE_POINTS)
        .into_par_iter()
        .fold(
            || HashMap::<_, usize>::default(),
            |mut frequencies, _| {
                let mut rng = thread_rng();
                let start = rng.gen_range(0..graph.node_count());
                let end = rng.gen_range(0..graph.node_count());
                let (_, path) =
                    petgraph::algo::astar(&graph, start.into(), |finish| finish == end.into(), |_| 1, |_| 0).unwrap();
                path.windows(2)
                    .map(|window| graph.edges_connecting(window[0], window[1]).next().unwrap())
                    .for_each(|edge| {
                        *frequencies.entry(edge.id()).or_default() += 1;
                    });
                frequencies
            },
        )
        .reduce(
            || HashMap::<_, usize>::default(),
            |mut frequencies, other_frequencies| {
                for (edge, count) in other_frequencies {
                    *frequencies.entry(edge).or_default() += count;
                }
                frequencies
            },
        );

    let mut frequencies = frequencies.into_iter().collect::<Vec<_>>();
    frequencies.sort_by_key(|(_, count)| *count);
    for (edge, _) in frequencies.iter().rev().take(3) {
        graph.remove_edge(*edge);
    }

    let components = petgraph::algo::kosaraju_scc(&graph);
    components.iter().map(|c| c.len()).product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const RIGHT_ANSWER: usize = 592_171;
    const TRIALS: usize = 1000;

    #[test]
    fn works_90_percent_of_the_time() {
        let correct = (0..TRIALS)
            .map(|_| do_solve(load_input()))
            .filter(|answer| *answer == RIGHT_ANSWER)
            .count();
        eprintln!("{:.2}% correct", correct * 100 / TRIALS);
        assert!(correct > TRIALS * 9 / 10);
    }
}
