use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let n = input();
    let mut adjacency_list = vec![Vec::new(); n + 1];

    for i in 1..=n {
        for dislike in (0..input()).map(|_| input()) {
            adjacency_list[i].push(dislike);
        }
    }

    let mut visited = vec![None; n + 1];

    for start in 1..=n {
        if visited[start].is_some() {
            continue;
        }

        visited[start] = Some(true);
        let mut stack = vec![start];

        while let Some(node) = stack.pop() {
            let color = visited[node].unwrap();

            for &adj in adjacency_list[node].iter() {
                if visited[adj].is_some() {
                    continue;
                }

                visited[adj] = Some(!color);
                stack.push(adj);
            }
        }
    }

    let (mut blue, mut white) = (Vec::new(), Vec::new());

    for (i, &color) in visited.iter().enumerate().skip(1) {
        if color.unwrap() {
            blue.push(i);
        } else {
            white.push(i);
        }
    }

    println!("{}", blue.len());

    for num in blue {
        print!("{num} ");
    }

    println!("\n{}", white.len());

    for num in white {
        print!("{num} ");
    }
}
