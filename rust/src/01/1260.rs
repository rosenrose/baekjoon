use std::collections::VecDeque;
use std::io;

enum Ops {
    DFS,
    BFS,
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m, v) = (input(), input(), input());

    let mut adjacency_list = (0..m).fold(vec![Vec::new(); n + 1], |mut acc, _| {
        let (v1, v2) = (input(), input());
        acc[v1].push(v2);
        acc[v2].push(v1);

        acc
    });

    for list in adjacency_list.iter_mut() {
        (*list).sort();
    }
    // println!("{adjacency_list:?}");
    search(&adjacency_list, v, Ops::DFS);
    println!("");
    search(&adjacency_list, v, Ops::BFS);
}

fn search(graph: &Vec<Vec<usize>>, start: usize, op: Ops) {
    let mut queue = VecDeque::from([start]);
    let mut visited = vec![false; graph.len()];

    while let Some(node) = match op {
        Ops::DFS => queue.pop_back(),
        Ops::BFS => queue.pop_front(),
    } {
        if visited[node] {
            continue;
        }

        print!("{node} ");
        visited[node] = true;

        match op {
            Ops::DFS => {
                for &neighbor in graph[node].iter().rev() {
                    queue.push_back(neighbor);
                }
            }
            Ops::BFS => {
                for &neighbor in graph[node].iter() {
                    queue.push_back(neighbor);
                }
            }
        }
    }
}
