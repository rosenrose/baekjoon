use std::collections::{HashSet, VecDeque};
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    const MAX_DIST: u32 = 50 * 20;
    let get_distance =
        |src: (i32, i32), dst: (i32, i32)| src.0.abs_diff(dst.0) + src.1.abs_diff(dst.1);

    'outer: for _ in 0..input() {
        let n = input();
        let house = (input(), input());
        let convenients: Vec<_> = (0..n).map(|_| (input(), input())).collect();
        let festival = (input(), input());

        let mut visited = HashSet::from([house]);
        let mut queue = VecDeque::from([house]);

        while let Some((x, y)) = queue.pop_front() {
            if get_distance((x, y), festival) <= MAX_DIST {
                println!("happy");
                continue 'outer;
            }

            for &conv in &convenients {
                if visited.contains(&conv) || get_distance((x, y), conv) > MAX_DIST {
                    continue;
                }

                visited.insert(conv);
                queue.push_back(conv);
            }
        }

        println!("sad");
    }
}
