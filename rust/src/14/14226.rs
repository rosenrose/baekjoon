use std::collections::{HashSet, VecDeque};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let s: u16 = buf.trim().parse().unwrap();
    let mut visited = HashSet::from([(1, 0)]);
    let mut queue = VecDeque::from([((1, 0), 0)]);

    while let Some(((screen, clipboard), time)) = queue.pop_front() {
        if screen == s {
            println!("{time}");
            return;
        }

        let adjacents = [
            (screen, screen),
            (screen + clipboard, clipboard),
            (screen.saturating_sub(1), clipboard),
        ];

        for adj in adjacents {
            if visited.contains(&adj) {
                continue;
            }

            visited.insert(adj);
            queue.push_back((adj, time + 1));
        }
    }
}
