use std::io;

const MAX: usize = 100 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n: usize = input();
    let mut adjacency_list = [(); MAX].map(|_| Vec::new());

    for i in 1..=n {
        for dislike in (0..input()).map(|_| input()) {
            adjacency_list[i].push(dislike);
        }
    }

    let mut visited = [None; MAX];

    for start in 1..=n {
        if visited[start].is_some() {
            continue;
        }

        visited[start] = Some(true);
        let mut stack = vec![start];

        while let Some(node) = stack.pop() {
            let color = visited[node].unwrap();

            for &adj in &adjacency_list[node] {
                if visited[adj].is_some() {
                    continue;
                }

                visited[adj] = Some(!color);
                stack.push(adj);
            }
        }
    }

    let (mut blue, mut white) = ([0; MAX], [0; MAX]);
    let (mut blue_len, mut white_len) = (0, 0);

    for (i, &color) in visited[1..=n].iter().flatten().enumerate() {
        if color {
            blue[blue_len] = i + 1;
            blue_len += 1;
        } else {
            white[white_len] = i + 1;
            white_len += 1;
        }
    }

    println!("{}", blue_len);

    for num in &blue[..blue_len] {
        print!("{num} ");
    }

    println!("\n{}", white_len);

    for num in &white[..white_len] {
        print!("{num} ");
    }
}
