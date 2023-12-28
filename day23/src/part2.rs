use std::{collections::VecDeque, mem::swap};

use arrayvec::ArrayVec;
use rustc_hash::FxHashMap as HashMap;

type Node = u8;
type Cost = u16;
type States = HashMap<State, Cost>;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct State {
    paths: ArrayVec<(Node, Node), 8>,
}

impl State {
    fn insert(&mut self, edge: (Node, Node)) {
        let insertion_idx = self
            .paths
            .iter()
            .position(|&e| e > edge)
            .unwrap_or_else(|| self.paths.len());
        self.paths.insert(insertion_idx, edge);
    }

    fn remove(&mut self, edge: (Node, Node)) {
        let idx = self.paths.iter().position(|&e| e == edge).unwrap();
        self.paths.remove(idx);
    }

    fn find_edge_containing(&self, node: Node) -> Option<(Node, Node)> {
        self.paths.iter().find(|&&(u, v)| u == node || v == node).copied()
    }

    fn find_distinct_edges_containing(&self, node1: Node, node2: Node) -> Option<((Node, Node), (Node, Node))> {
        let mut edge1 = None;
        let mut edge2 = None;
        for &edge in self.paths.iter() {
            if edge.0 == node1 || edge.1 == node1 {
                edge1 = Some(edge);
                if edge2.is_some() {
                    break;
                }
            }
            if edge.0 == node2 || edge.1 == node2 {
                edge2 = Some(edge);
                if edge1.is_some() {
                    break;
                }
            }
        }
        edge1.zip(edge2).filter(|&(e1, e2)| e1 != e2)
    }
}

struct Solver {
    states: States,
    new_states: States,
    start: Node,
    goal: Node,
}

impl Solver {
    fn new(start: Node, goal: Node) -> Self {
        let mut states: States = Default::default();
        states.insert(Default::default(), 0);

        Self {
            states,
            new_states: Default::default(),
            start,
            goal,
        }
    }

    fn add_vertex(&mut self, vertex: Node) {
        debug_assert_eq!(self.new_states.len(), 0);
        for (mut state, cost) in self.states.drain() {
            state.insert((vertex, vertex));
            self.new_states.insert(state, cost);
        }
        swap(&mut self.states, &mut self.new_states);
    }

    fn remove_vertex(&mut self, vertex: Node) {
        debug_assert_eq!(self.new_states.len(), 0);
        if vertex == self.start || vertex == self.goal {
            return;
        }

        for (mut state, cost) in self.states.drain() {
            if let Some(edge) = state.find_edge_containing(vertex) {
                if edge == (vertex, vertex) {
                    state.remove(edge);
                    self.new_states
                        .entry(state)
                        .and_modify(|c| *c = (*c).max(cost))
                        .or_insert(cost);
                }
            } else {
                self.new_states
                    .entry(state)
                    .and_modify(|c| *c = (*c).max(cost))
                    .or_insert(cost);
            }
        }

        swap(&mut self.states, &mut self.new_states);
    }

    fn add_edge(&mut self, u: Node, v: Node, w: Cost) {
        debug_assert_eq!(self.new_states.len(), 0);
        let old_dp = self.states.clone();

        // Edges that involve the start node must be used. There's only one, so if we don't use it we've not got any way
        // to get to the end.
        if u == self.start || v == self.start {
            self.states.clear();
        }

        for (mut state, cost) in old_dp {
            let Some((u_edge, v_edge)) = state.find_distinct_edges_containing(u, v) else {
                continue;
            };
            let x = if u_edge.0 == u { u_edge.1 } else { u_edge.0 };
            let y = if v_edge.0 == v { v_edge.1 } else { v_edge.0 };
            let new_edge = if x < y { (x, y) } else { (y, x) };
            let new_cost = cost + w;

            state.remove(u_edge);
            state.remove(v_edge);
            state.insert(new_edge);
            self.states
                .entry(state)
                .and_modify(|c| *c = (*c).max(new_cost))
                .or_insert(new_cost);
        }
    }
}

pub fn solve(input: &str) -> Cost {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let area = width * height;

    let start_idx = 1;
    let goal_idx = area - 2;

    let occupancy = input
        .bytes()
        .filter(|&b| b != b'\n')
        .map(|b| b != b'#')
        .collect::<Vec<_>>();

    let adjacency = occupancy
        .iter()
        .enumerate()
        .map(|(idx, &is_empty)| {
            if !is_empty {
                return ArrayVec::<usize, 4>::new();
            }

            let mut neighbors = ArrayVec::new();

            let x = idx % width;
            let y = idx / width;

            if x != 0 && occupancy[idx - 1] {
                neighbors.push(idx - 1);
            }
            if y != 0 && occupancy[idx - width] {
                neighbors.push(idx - width);
            }
            if x != width - 1 && occupancy[idx + 1] {
                neighbors.push(idx + 1);
            }
            if y != height - 1 && occupancy[idx + width] {
                neighbors.push(idx + width);
            }
            neighbors.sort();

            neighbors
        })
        .collect::<Vec<_>>();

    let mut graph = vec![ArrayVec::<(usize, Cost), 4>::new(); area];

    let mut outer_visited = vec![false; area];
    let mut outer_queue: VecDeque<usize> = Default::default();
    outer_queue.push_back(start_idx);

    while let Some(source) = outer_queue.pop_front() {
        let mut visited = vec![false; area];
        let mut queue: VecDeque<(usize, Cost)> = Default::default();
        queue.push_back((source, 0));

        while let Some((node, cost)) = queue.pop_front() {
            let neighbors = &adjacency[node];

            if (neighbors.len() > 2 || node == start_idx || node == goal_idx) && node != source {
                graph[source].push((node, cost));
                if !outer_visited[node] {
                    outer_visited[node] = true;
                    outer_queue.push_back(node);
                }
            } else {
                for &neighbor in neighbors {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back((neighbor, cost + 1));
                    }
                }
            }
        }
    }

    // reduce graph indices to u8
    let mut i: Node = 0;
    let mut new_start_idx = 0;
    let mut new_goal_idx = 0;
    let mut new_indices = vec![0; area];
    for (idx, neighbors) in graph.iter_mut().enumerate() {
        if !neighbors.is_empty() {
            if idx == start_idx {
                new_start_idx = i;
            }
            if idx == goal_idx {
                new_goal_idx = i;
            }
            new_indices[idx] = i;
            i += 1;
        }
    }
    let mut new_graph = vec![ArrayVec::<(Node, Cost), 4>::new(); usize::from(i)];
    for (idx, neighbors) in graph.iter_mut().enumerate() {
        if !neighbors.is_empty() {
            for &(neighbor, weight) in neighbors.iter() {
                new_graph[usize::from(new_indices[idx])].push((new_indices[neighbor], weight));
            }
        }
    }
    let graph = new_graph;
    let start_idx = new_start_idx;
    let goal_idx = new_goal_idx;

    let mut solver = Solver::new(start_idx, goal_idx);

    let mut q = VecDeque::new();
    let mut visited = vec![false; area];
    let mut known = vec![false; area];
    q.push_back(start_idx);
    visited[usize::from(start_idx)] = true;

    let mut unexplored = graph.iter().map(|neighbors| neighbors.len()).collect::<Vec<_>>();

    while let Some(node) = q.pop_front() {
        solver.add_vertex(node);
        known[usize::from(node)] = true;

        for &(neighbor, weight) in &graph[usize::from(node)] {
            if known[usize::from(neighbor)] {
                solver.add_edge(node, neighbor, weight);
                unexplored[usize::from(node)] -= 1;
                unexplored[usize::from(neighbor)] -= 1;
                if unexplored[usize::from(node)] == 0 {
                    solver.remove_vertex(node);
                }
                if unexplored[usize::from(neighbor)] == 0 {
                    solver.remove_vertex(neighbor);
                }
            } else if !visited[usize::from(neighbor)] {
                visited[usize::from(neighbor)] = true;
                q.push_back(neighbor);
            }
        }
    }

    let mut sol_set = State::default();
    sol_set.insert((start_idx, goal_idx));
    solver.states[&sol_set]
}
