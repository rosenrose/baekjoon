use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let money = input.next().unwrap();
    input.next();

    let coins: Vec<_> = input.collect();
    let biggest = *coins.iter().max().unwrap();

    let distances = dijkstra(&coins, biggest);

    println!(
        "{}",
        money / biggest + distances[(money % biggest) as usize]
    );
}

fn dijkstra(coins: &Vec<i64>, biggest: i64) -> Vec<i64> {
    let mut distances = vec![i64::MAX; biggest as usize];
    distances[0] = 0;

    let mut queue = BinaryHeap::from([Reverse((0, 0))]);

    while !queue.is_empty() {
        let (a, current_coin) = queue.pop().unwrap().0;
        let b = distances[current_coin as usize];

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
            queue.push(Reverse((new_dist, next)));
        }
    }

    distances
}
