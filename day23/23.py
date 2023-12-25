# code refactored/adapted from @hegdahl
# https://parameterized-algorithms.mimuw.edu.pl/parameterized-algorithms.pdf#chapter.7

DIRECTIONS = [
    (0, -1),  # Left
    (-1, 0),  # Up
    (0, 1),  # Right
    (1, 0),  # Down
]


class Solver:
    def __init__(self, start_idx: int, end_idx: int):
        self.states: dict[frozenset[tuple[int, int]], int] = {frozenset(): 0}
        self.start_idx = start_idx
        self.end_idx = end_idx

    def add_vertex(self, v: int):
        new_states = {}
        for state, value in self.states.items():
            new_states[state | {(v, v)}] = value
        self.states = new_states

    def remove_vertex(self, v: int):
        if v in (self.start_idx, self.end_idx):
            return

        new_states = {}
        for state, length in self.states.items():
            for edge in state:
                if v in edge:
                    if edge == (v, v):
                        new_state = state - {(v, v)}
                        previous_best = new_states.get(new_state, 0)
                        if length > previous_best:
                            new_states[new_state] = length
                    break
            else:
                previous_best = new_states.get(state, 0)
                if length > previous_best:
                    new_states[state] = length
        self.states = new_states

    def add_edge(self, u: int, v: int, weight: int):
        # NB: we can also not include (u, v) in the states, so this function implicitly leaves
        # everything that was in the states before unchanged
        old_dp = list(self.states.items())
        for state, value in old_dp:
            u_edge = None
            v_edge = None

            for edge in state:
                if u in edge:
                    u_edge = edge
                if v in edge:
                    v_edge = edge
            if u_edge is None or v_edge is None or u_edge == v_edge:
                continue

            # we've got (u, x) and (v, y) -> we contract (u, v) to (x, y)
            x = u_edge[0] if u_edge[0] != u else u_edge[1]
            y = v_edge[0] if v_edge[0] != v else v_edge[1]
            new_edge = (min(x, y), max(x, y))
            new_state = (state - {u_edge, v_edge}) | {new_edge}
            new_value = value + weight

            previous_best = self.states.get(new_state, 0)
            if new_value > previous_best:
                self.states[new_state] = new_value


class Input:
    def __init__(self):
        self.is_empty = [char != "#" for row in map(str.strip, open("src/input.txt")) for char in row]
        self.width = len(open("src/input.txt").readline().strip())
        self.height = len(self.is_empty) // self.width

        self.adjacency = [
            [
                ni * self.width + nj
                for di, dj in DIRECTIONS
                if (
                    (ni := i + di) in range(self.height)
                    and (nj := j + dj) in range(self.width)
                    and self.is_empty[ni * self.width + nj]
                )
            ]
            if self.is_empty[i * self.width + j]
            else []
            for i in range(self.height)
            for j in range(self.width)
        ]

    def create_1d_bitset(self):
        return [False] * (self.height * self.width)

    def is_start(self, idx):
        return idx in range(self.width)

    def is_end(self, idx):
        return idx in range((self.height - 1) * self.width, self.height * self.width)

def dfs(inp, start_idx):
    inner_visited = inp.create_1d_bitset()

    def inner(idx, d):
        inner_visited[idx] = True

        neighbors = len(inp.adjacency[idx])

        if (neighbors > 2 or inp.is_start(idx) or inp.is_end(idx)) and idx != start_idx:
            yield (idx, d)
        else:
            for nidx in inp.adjacency[idx]:
                if inner_visited[nidx]:
                    continue

                yield from inner(nidx, d + 1)

    return inner(start_idx, 0)


def construct_edge_contracted_graph(inp):
    visited = inp.create_1d_bitset()

    graph = [[] for _ in range(inp.height * inp.width + 1)]

    queue = []
    for j in range(inp.width):
        if inp.is_empty[j]:
            queue.append(j)
            break

    for sidx in queue:  # while there are elements in queue
        for oidx, d in dfs(inp, sidx):
            graph[sidx].append((oidx, d))
            if not visited[oidx]:
                visited[oidx] = True
                queue.append(oidx)

    return graph


def find_start_and_end(inp):
    start_idx, end_idx = None, None
    for next in range(inp.width):
        if inp.is_empty[0 +next]:
            start_idx = next
        if inp.is_empty[(inp.height - 1) * inp.width + next]:
            end_idx = (inp.height - 1) * inp.width + next
    assert start_idx is not None
    assert end_idx is not None
    return start_idx, end_idx


def main() -> None:
    inp = Input()
    graph = construct_edge_contracted_graph(inp)
    start_idx, end_idx = find_start_and_end(inp)

    dp_manager = Solver(start_idx, end_idx)

    q = [start_idx]
    visited = inp.create_1d_bitset()
    known = inp.create_1d_bitset()
    visited[start_idx] = True

    unexplored_edges = [len(neighbors) for neighbors in graph]


    for current in q:
        dp_manager.add_vertex(current)
        known[current] = True

        for next, w in graph[current]:
            if known[next]:
                dp_manager.add_edge(current, next, w)
                unexplored_edges[current] -= 1
                unexplored_edges[next] -= 1
                if unexplored_edges[current] == 0:
                    dp_manager.remove_vertex(current)
                if unexplored_edges[next] == 0:
                    dp_manager.remove_vertex(next)
            elif not visited[next]:
                visited[next] = True
                q.append(next)

    print(dp_manager.states[frozenset({(start_idx, end_idx)})])


if __name__ == "__main__":
    main()
