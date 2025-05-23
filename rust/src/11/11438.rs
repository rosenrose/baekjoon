use std::fmt::Write;
use std::io;

const NODES_MAX: usize = 100_000 + 1;
const PARENTS_MAX: usize = 17;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = input();
    let mut adjacency_list = [(); NODES_MAX].map(|_| Vec::new());

    for (a, b) in (0..n - 1).map(|_| (input(), input())) {
        adjacency_list[a].push(b as i32);
        adjacency_list[b].push(a as i32);
    }

    let mut depths = [0; NODES_MAX];
    let mut parents = [[0; PARENTS_MAX]; NODES_MAX];
    let root = 1;
    let mut stack = vec![(root, 1)];

    while let Some((node, depth)) = stack.pop() {
        depths[node] = depth;

        for &adj in &adjacency_list[node] {
            if depths[adj as usize] != 0 {
                continue;
            }

            parents[adj as usize][0] = node as i32;
            stack.push((adj as usize, depth + 1));
        }

        for i in 1..PARENTS_MAX {
            let parent = parents[node][i - 1] as usize;

            if parent == 0 {
                break;
            }

            parents[node][i] = parents[parent][i - 1];
        }
    }

    for (u, v) in (0..input()).map(|_| (input(), input())) {
        writeln!(output, "{}", lca((u, v), &depths[..=n], &parents[..=n])).unwrap();
    }

    print!("{output}");
}

fn lca((mut u, mut v): (usize, usize), depths: &[i32], parents: &[[i32; PARENTS_MAX]]) -> usize {
    if depths[v] > depths[u] {
        (u, v) = (v, u);
    }

    while depths[u] != depths[v] {
        u = *parents[u]
            .iter()
            .rfind(|&&p| depths[p as usize] >= depths[v])
            .unwrap() as usize;
    }

    if u == v {
        return u;
    }

    while let Some(i) = (0..PARENTS_MAX)
        .rev()
        .find(|&i| parents[u][i] != parents[v][i])
    {
        u = parents[u][i] as usize;
        v = parents[v][i] as usize;
    }

    parents[u][0] as usize
}
