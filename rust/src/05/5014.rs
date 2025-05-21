use std::collections::VecDeque;
use std::io;

const MAX: usize = 1_000_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let [total_floor, start, startlink, up, down] = [(); 5].map(|_| input.next().unwrap());
    let mut visited = [false; MAX];
    visited[start as usize] = true;

    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((floor, count)) = queue.pop_front() {
        if floor == startlink {
            println!("{count}");
            return;
        }

        for &adj in [floor + up, floor - down]
            .iter()
            .filter(|&&adj| 1 <= adj && adj <= total_floor)
        {
            if visited[adj as usize] {
                continue;
            }

            visited[adj as usize] = true;
            queue.push_back((adj, count + 1));
        }
    }

    println!("use the stairs");
}
