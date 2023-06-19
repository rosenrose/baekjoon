use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let (money, _) = (input.next().unwrap(), input.next());
    let coins: Vec<_> = input.collect();
    let biggest = *coins.iter().max().unwrap();

    let (start, end) = (0, money % biggest);
    let distance = dijkstra(&coins, biggest, start, end);

    println!("{}", (money / biggest) + distance);
}

fn dijkstra(coins: &[i64], biggest: i64, start: i64, end: i64) -> i64 {
    let mut distances = vec![i64::MAX; biggest as usize];
    distances[start as usize] = 0;

    let mut queue = BinaryHeap::from([(Reverse(0), start)]);

    while let Some((Reverse(a), current_coin)) = queue.pop() {
        let b = distances[current_coin as usize];

        if current_coin == end {
            return b;
        }
        if a != b {
            continue;
        }

        for coin in coins {
            let c = current_coin + coin;
            let next = c % biggest;
            let new_dist = a + 1 - c / biggest;

            if distances[next as usize] <= new_dist {
                continue;
            }

            distances[next as usize] = new_dist;
            queue.push((Reverse(new_dist), next));
        }
    }

    distances[end as usize]
}
