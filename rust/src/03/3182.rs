use std::io;

const MAX: usize = 1000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut graph = [0; MAX];

    for (i, num) in input.enumerate() {
        graph[i + 1] = num;
    }

    let (mut first_person, mut max_count) = (0, 0);

    for start in 1..=n {
        let mut count = 0;
        let mut visited = [false; MAX];
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
