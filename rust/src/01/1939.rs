use std::collections::VecDeque;
use std::io;

const NODES_MAX: usize = 10000 + 1;
const EDGES_MAX: usize = 100_000;

struct DisjointSet<const N: usize>([i32; N]);

impl<const N: usize> DisjointSet<N> {
    fn make() -> Self {
        Self(std::array::from_fn(|i| i as i32))
    }

    fn find(&mut self, a: i32) -> i32 {
        let a_idx = a as usize;

        if self.0[a_idx] != a {
            self.0[a_idx] = self.find(self.0[a_idx]);
        }

        self.0[a_idx]
    }

    fn union(&mut self, a: i32, b: i32) {
        let (a, b) = (self.find(a), self.find(b));

        if a == b {
            return;
        }

        self.0[b as usize] = a;
    }

    fn is_same(&mut self, a: i32, b: i32) -> bool {
        self.find(a) == self.find(b)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut disjoint_set = DisjointSet::<NODES_MAX>::make();
    let mut edges = [[0; 3]; EDGES_MAX];

    for (i, edge) in (0..m).map(|_| [(); 3].map(|_| input())).enumerate() {
        edges[i] = edge;
    }

    edges[..m as usize].sort_unstable_by_key(|&[.., weight]| weight);

    let (start, end) = (input(), input());

    for &[a, b, c] in edges.iter().rev() {
        disjoint_set.union(a, b);

        if disjoint_set.is_same(start, end) {
            println!("{c}");
            return;
        }
    }

    // let mut adjacency_array = ([i32::MAX; NODES_MAX], [((0, 0), 0); EDGES_MAX * 2]);
    // let (mut min_weight, mut max_weight) = (i32::MAX, 0);

    // for (i, [a, b, c]) in (0..m).map(|i| (i << 1, [(); 3].map(|_| input()))) {
    //     let prev = adjacency_array.0[a as usize];
    //     adjacency_array.0[a as usize] = i;
    //     adjacency_array.1[i as usize] = ((b, c), prev);

    //     let prev = adjacency_array.0[b as usize];
    //     adjacency_array.0[b as usize] = i + 1;
    //     adjacency_array.1[(i + 1) as usize] = ((a, c), prev);

    //     (min_weight, max_weight) = (c.min(min_weight), c.max(max_weight));
    // }

    // let (start, end) = (input(), input());

    // println!(
    //     "{}",
    //     binary_search(
    //         &adjacency_array,
    //         min_weight,
    //         max_weight,
    //         start as usize,
    //         end as usize
    //     )
    // );
}

fn binary_search(
    graph: &([i32; NODES_MAX], [((i32, i32), i32); EDGES_MAX * 2]),
    mut lo: i32,
    mut hi: i32,
    start: usize,
    end: usize,
) -> i32 {
    let mut result = 0;

    while lo <= hi {
        let mid = lo + ((hi - lo) >> 1);

        if bfs(graph, start, end, mid) {
            result = mid;
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    result
}

fn bfs(
    (nodes, edges): &([i32; NODES_MAX], [((i32, i32), i32); EDGES_MAX * 2]),
    start: usize,
    end: usize,
    weight: i32,
) -> bool {
    let mut visited = [0; NODES_MAX];
    visited[start] = i32::MAX;

    let mut queue = VecDeque::from([(start as i32, i32::MAX)]);

    while let Some((node, min_weight)) = queue.pop_front() {
        let mut edge = nodes[node as usize];

        while let Some(&((adj, bridge_weight), next_edge)) = edges.get(edge as usize) {
            let new_weight = weight.min(min_weight);

            if visited[adj as usize] < new_weight && bridge_weight >= weight {
                if adj as usize == end {
                    return true;
                }

                visited[adj as usize] = new_weight;
                queue.push_back((adj, new_weight));
            }

            edge = next_edge;
        }
    }

    false
}
