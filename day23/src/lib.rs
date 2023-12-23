use std::fmt::Display;

use petgraph::{algo::all_simple_paths, graphmap::DiGraphMap, visit::Bfs};

struct Length(usize);

impl<T> FromIterator<T> for Length {
    fn from_iter<I: IntoIterator>(iter: I) -> Self {
        Self(iter.into_iter().count())
    }
}

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

fn do_solve(input: &str) -> usize {
    let mut graph = DiGraphMap::<(usize, usize), ()>::new();

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
                        start = Some((x, y));
                    } else if y == height - 1 {
                        goal = Some((x, y));
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
                graph.add_edge((x, y), (nx, ny), ());
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

    let graph = graph.into_graph::<usize>();
    let start = graph.node_indices().find(|&n| graph[n] == start).unwrap();
    let goal = graph.node_indices().find(|&n| graph[n] == goal).unwrap();

    let answer = all_simple_paths(&graph, start, goal, 0, None)
        .map(|Length(l)| l - 1)
        .max()
        .unwrap();
    answer
}
