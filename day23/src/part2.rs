use std::{collections::VecDeque, hash::BuildHasherDefault};
use std::mem::swap;

use arrayvec::ArrayVec;
use rustc_hash::{FxHashMap as HashMap, FxHasher};

type HashSet<T> = im::HashSet<T, BuildHasherDefault<FxHasher>>;

type Node = usize;
type Cost = u16;
type States = HashMap<HashSet<(Node, Node)>, Cost>;

struct Solver {
    states: States,
    new_states: States,
    start: Node,
    goal: Node,
}

impl Solver {
    fn new(start: Node, goal: Node) -> Self {
        let mut states: States = Default::default();
        states.insert(HashSet::default(), 0);

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
            if let Some(&edge) = state.iter().find(|&&(a, b)| a == vertex || b == vertex) {
                if edge == (vertex, vertex) {
                    state.remove(&edge);
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
        for (mut state, cost) in old_dp {
            let mut u_edge = None;
            let mut v_edge = None;
            for &edge in state.iter() {
                if edge.0 == u || edge.1 == u {
                    u_edge = Some(edge);
                }
                if edge.0 == v || edge.1 == v {
                    v_edge = Some(edge);
                }
            }
            let Some(u_edge) = u_edge else {
                continue;
            };
            let Some(v_edge) = v_edge else {
                continue;
            };
            if u_edge == v_edge {
                continue;
            }
            let x = if u_edge.0 == u { u_edge.1 } else { u_edge.0 };
            let y = if v_edge.0 == v { v_edge.1 } else { v_edge.0 };
            let new_edge = if x < y { (x, y) } else { (y, x) };
            let new_cost = cost + w;
            state.remove(&u_edge);
            state.remove(&v_edge);
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

    let mut graph = vec![ArrayVec::<(Node, Cost), 4>::new(); area];

    let mut outer_visited = vec![false; area];
    let mut outer_queue: VecDeque<Node> = Default::default();
    outer_queue.push_back(start_idx);

    while let Some(source) = outer_queue.pop_front() {
        let mut visited = vec![false; area];
        let mut queue: VecDeque<(Node, Cost)> = Default::default();
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

    let mut solver = Solver::new(start_idx, goal_idx);

    let mut q = VecDeque::new();
    let mut visited = vec![false; area];
    let mut known = vec![false; area];
    q.push_back(start_idx);
    visited[start_idx] = true;


    let mut unexplored = graph.iter().map(|neighbors| neighbors.len()).collect::<Vec<_>>();

    while let Some(node) = q.pop_front() {
        solver.add_vertex(node);
        known[node] = true;

        for &(neighbor, weight) in &graph[node] {
            if known[neighbor] {
                solver.add_edge(node, neighbor, weight);
                unexplored[node] -= 1;
                unexplored[neighbor] -= 1;
                if unexplored[node] == 0 {
                    solver.remove_vertex(node);
                }
                if unexplored[neighbor] == 0 {
                    solver.remove_vertex(neighbor);
                }
            } else if !visited[neighbor] {
                visited[neighbor] = true;
                q.push_back(neighbor);
            }
        }
    }

    let mut sol_set = HashSet::default();
    sol_set.insert((start_idx, goal_idx));
    solver.states[&sol_set]
}
