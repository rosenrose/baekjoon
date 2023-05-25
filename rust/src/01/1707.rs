use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    'outer: for _ in 0..input() {
        let (v, e) = (input() as usize, input());
        let mut adjacency_list = vec![Vec::new(); v + 1];

        for (a, b) in (0..e).map(|_| (input() as i16, input() as i16)) {
            adjacency_list[a as usize].push(b);
            adjacency_list[b as usize].push(a);
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

                for &adj in adjacency_list[node as usize].iter() {
                    if let Some(adj_color) = visited[adj as usize] {
                        if adj_color == color {
                            println!("NO");
                            continue 'outer;
                        }

                        continue;
                    }

                    visited[adj as usize] = Some(!color);
                    stack.push(adj);
                }
            }
        }

        println!("YES");
    }
}
