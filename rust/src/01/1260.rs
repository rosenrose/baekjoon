use std::collections::{HashSet, VecDeque};
use std::io::{stdin, Read};

enum Ops {
    DFS,
    BFS,
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let (n, m, v) = (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
    );

    let mut adjacency_list = (0..m).fold(vec![Vec::new(); n + 1], |mut acc, _| {
        let (v1, v2) = (input.next().unwrap(), input.next().unwrap());
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
    let mut visited = HashSet::new();

    while !queue.is_empty() {
        let node = match op {
            Ops::DFS => queue.pop_back(),
            Ops::BFS => queue.pop_front(),
        }
        .unwrap();

        if visited.contains(&node) {
            continue;
        }

        print!("{node} ");
        visited.insert(node);

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
