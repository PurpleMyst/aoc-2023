use std::fmt::Display;

use petgraph::{algo::all_simple_paths, graphmap::DiGraphMap, prelude::*};

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let input = include_str!("input.txt");

    let part1 = do_solve(input);

    let part2_input = input
        .replace('>', ".")
        .replace('<', ".")
        .replace('^', ".")
        .replace('v', ".");
    let part2 = do_solve(&part2_input);

    (part1, part2)
}

fn do_solve(input: &str) -> u16 {
    let mut graph = DiGraphMap::<(u8, u8), u16>::new();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid = grid::Grid::from_vec(input.bytes().filter(|&b| !b.is_ascii_whitespace()).collect(), width);
    let mut start = None;
    let mut goal = None;

    for (y, row) in grid.iter_rows().enumerate() {
        for (x, c) in row.enumerate() {
            let dot_neighbors: [_; 4];
            let arrow_neighbor;

            let neighbors = match c {
                b'.' => {
                    dot_neighbors = [
                        y.checked_sub(1).map(|y| (x, y)),
                        x.checked_sub(1).map(|x| (x, y)),
                        Some((x + 1, y)),
                        Some((x, y + 1)),
                    ];

                    if y == 0 {
                        start = Some((x as u8, y as u8));
                    } else if y == height - 1 {
                        goal = Some((x as u8, y as u8));
                    }

                    &dot_neighbors[..]
                }

                b'>' => {
                    arrow_neighbor = Some((x + 1, y));
                    std::array::from_ref(&arrow_neighbor)
                }

                b'v' => {
                    arrow_neighbor = Some((x, y + 1));
                    std::array::from_ref(&arrow_neighbor)
                }

                b'<' => {
                    arrow_neighbor = y.checked_sub(1).map(|y| (x, y));
                    std::array::from_ref(&arrow_neighbor)
                }

                b'^' => {
                    arrow_neighbor = y.checked_sub(1).map(|y| (x, y));
                    std::array::from_ref(&arrow_neighbor)
                }

                b'#' => continue,

                _ => panic!("Unknown character: {}", *c as char),
            };

            for (nx, ny) in neighbors
                .iter()
                .filter_map(|&n| n)
                .filter(|&(nx, ny)| !matches!(grid.get(ny, nx), None | Some(b'#')))
            {
                graph.add_edge((x as u8, y as u8), (nx as u8, ny as u8), 1);
            }
        }
    }
    let start = start.unwrap();
    let goal = goal.unwrap();

    // save path as dot
    use petgraph::dot::{Config, Dot};
    std::fs::write(
        "graph.dot",
        format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel])),
    )
    .unwrap();

    let mut graph = graph.into_graph::<usize>();

    'consolidate_loop: loop {
        for node in graph.node_indices() {
            if graph.neighbors(node).count() == 2 {
                let mut edges = graph.edges(node);
                let from = edges.next().unwrap();
                debug_assert_ne!(from.target(), node);
                let to = edges.next().unwrap();
                debug_assert_ne!(to.target(), node);

                let new_weight = from.weight() + to.weight();
                let from_target = from.target();
                let to_target = to.target();
                graph.add_edge(from_target, to_target, new_weight);
                graph.add_edge(to_target, from_target, new_weight);
                graph.remove_node(node);
                continue 'consolidate_loop;
            }
        }
        break;
    }

    let start = graph.node_indices().find(|&n| graph[n] == start).unwrap();
    let goal = graph.node_indices().find(|&n| graph[n] == goal).unwrap();

    let answer = all_simple_paths(&graph, start, goal, 0, None)
        .map(|path: Vec<_>| {
            path.windows(2)
                .map(|w| graph.edge_weight(graph.find_edge(w[0], w[1]).unwrap()).unwrap())
                .sum::<u16>()
        })
        .max()
        .unwrap();
    answer
}
