use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let graph: Vec<_> = buf.lines().flat_map(str::parse::<usize>).collect();

    let (mut first_person, mut max_count) = (0, 0);

    for start in 1..graph.len() {
        let mut count = 0;
        let mut visited = vec![false; graph.len()];
        visited[start] = true;

        let mut node = start;

        loop {
            count += 1;
            let adj = graph[node];

            if visited[adj] {
                break;
            }

            visited[adj] = true;
            node = adj;
        }

        if count > max_count {
            max_count = count;
            first_person = start;
        }
    }

    println!("{first_person}");
}
