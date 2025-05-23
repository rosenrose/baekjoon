use std::collections::VecDeque;
use std::io;

enum Ops {
    DFS,
    BFS,
}

const MAX: usize = 1000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let [n, m, v] = [(); 3].map(|_| input());
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for (v1, v2) in (0..m).map(|_| (input(), input())) {
        adjacency_list[v1].push(v2);
        adjacency_list[v2].push(v1);
    }

    for list in &mut adjacency_list[..=n] {
        (*list).sort();
    }

    for node in search(&adjacency_list, v, Ops::DFS)
        .iter()
        .take_while(|&&node| node != 0)
    {
        print!("{node} ");
    }

    println!("");

    for node in search(&adjacency_list, v, Ops::BFS)
        .iter()
        .take_while(|&&node| node != 0)
    {
        print!("{node} ");
    }
}

fn search(graph: &[Vec<usize>], start: usize, op: Ops) -> [usize; MAX] {
    let mut path = [0; MAX];
    let mut visited = [false; MAX];
    let mut queue = VecDeque::from([start]);
    let mut i = 0;

    while let Some(node) = match op {
        Ops::DFS => queue.pop_back(),
        Ops::BFS => queue.pop_front(),
    } {
        if visited[node] {
            continue;
        }

        visited[node] = true;
        path[i] = node;
        i += 1;

        match op {
            Ops::DFS => {
                for &adj in graph[node].iter().rev() {
                    queue.push_back(adj);
                }
            }
            Ops::BFS => {
                for &adj in &graph[node] {
                    queue.push_back(adj);
                }
            }
        }
    }

    path
}
