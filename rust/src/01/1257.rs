use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

const COINS_MAX: usize = 1000;
const MONEY_MAX: usize = 10000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [money, n] = [(); 2].map(|_| input.next().unwrap());
    let mut coins = [0; COINS_MAX];

    for (i, coin) in input.enumerate() {
        coins[i] = coin;
    }

    let biggest = *coins[..n].iter().max().unwrap();

    let (start, end) = (0, money % biggest);
    let distance = dijkstra(&coins, biggest, start, end);

    println!("{}", (money / biggest) + distance);
}

fn dijkstra(coins: &[usize], biggest: usize, start: usize, end: usize) -> usize {
    let mut distances = [usize::MAX; MONEY_MAX];
    distances[start as usize] = 0;

    let mut queue = BinaryHeap::from([(Reverse(0), start)]);

    while let Some((Reverse(a), current_coin)) = queue.pop() {
        let b = distances[current_coin];

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

            if distances[next] <= new_dist {
                continue;
            }

            distances[next] = new_dist;
            queue.push((Reverse(new_dist), next));
        }
    }

    distances[end]
}
