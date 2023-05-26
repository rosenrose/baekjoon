use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    'outer: for _ in 0..input() {
        let (v, e) = (input(), input());
        let mut adjacency_array = (vec![i32::MAX; v + 1], vec![(0, 0); e << 1]);

        for (i, (a, b)) in (0..e).map(|i| (i << 1, (input(), input()))) {
            let prev = adjacency_array.0[a];
            adjacency_array.0[a] = i as i32;
            adjacency_array.1[i] = (b as i16, prev);

            let prev = adjacency_array.0[b];
            adjacency_array.0[b] = (i + 1) as i32;
            adjacency_array.1[i + 1] = (a as i16, prev);
        }

        let mut visited = vec![None; v + 1];

        for start in 1..=v {
            if visited[start].is_some() {
                continue;
            }

            visited[start] = Some(true);
            let mut stack = vec![start as i16];

            while let Some(node) = stack.pop() {
                let color = visited[node as usize].unwrap();
                let mut edge = adjacency_array.0[node as usize];

                while let Some(&(adj, next_edge)) = adjacency_array.1.get(edge as usize) {
                    if let Some(adj_color) = visited[adj as usize] {
                        if adj_color == color {
                            println!("NO");
                            continue 'outer;
                        }
                    } else {
                        visited[adj as usize] = Some(!color);
                        stack.push(adj);
                    }

                    edge = next_edge;
                }
            }
        }

        println!("YES");
    }
}