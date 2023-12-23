use petgraph::prelude::*;

type CondensedGraph = UnGraph<(u8, u8), u16, u16>;

fn do_search(
    graph: &CondensedGraph,
    current: NodeIndex<u16>,
    goal: NodeIndex<u16>,
    distance: u16,
    mut visited: u64,
) -> u16 {
    if current == goal {
        return distance;
    }
    visited |= 1 << current.index();
    graph
        .edges(current)
        .filter(|&edge| visited & (1 << edge.target().index()) == 0)
        .map(|edge| do_search(graph, edge.target(), goal, distance + edge.weight(), visited))
        .max()
        .unwrap_or(0)
}

pub fn solve(input: &str) -> u16 {
    let mut graph = UnGraphMap::<(u8, u8), u16>::new();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid = grid::Grid::from_vec(input.bytes().filter(|&b| !b.is_ascii_whitespace()).collect(), width);
    let mut start = None;
    let mut goal = None;

    for (y, row) in grid.iter_rows().enumerate() {
        for (x, c) in row.enumerate() {
            let neighbors = match c {
                b'.' => {
                    if y == 0 {
                        start = Some((x as u8, y as u8));
                    } else if y == height - 1 {
                        goal = Some((x as u8, y as u8));
                    }

                    [
                        y.checked_sub(1).map(|y| (x, y)),
                        x.checked_sub(1).map(|x| (x, y)),
                        Some((x + 1, y)),
                        Some((x, y + 1)),
                    ]
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

    let mut graph = graph.into_graph::<u16>();

    'consolidate_loop: loop {
        for node in graph.node_indices() {
            match graph.neighbors(node).count() {
                2 => {
                    let mut edges = graph.edges(node);
                    let from = edges.next().unwrap();
                    debug_assert_ne!(from.target(), node);
                    let to = edges.next().unwrap();
                    debug_assert_ne!(to.target(), node);

                    let new_weight = from.weight() + to.weight();
                    let from_target = from.target();
                    let to_target = to.target();
                    graph.add_edge(from_target, to_target, new_weight);
                    graph.remove_node(node);
                    continue 'consolidate_loop;
                }
                _ => (),
            }
        }
        break;
    }

    let start = graph.node_indices().find(|&n| graph[n] == start).unwrap();
    let goal = graph.node_indices().find(|&n| graph[n] == goal).unwrap();

    do_search(&graph, start, goal, 0, 0)
}
