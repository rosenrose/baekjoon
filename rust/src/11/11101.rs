use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    for (conditons, combinations) in (0..parse_int(input())).map(|_| (input(), input())) {
        let times: HashMap<_, _> = conditons
            .split(',')
            .map(|condition| {
                let (name, time) = condition.split_once(':').unwrap();
                (name, parse_int(time))
            })
            .collect();
        // println!("{costs:?}");
        let min_time = combinations
            .split('|')
            .map(|combi| {
                let max = combi.split('&').map(|name| times[name]).max().unwrap();
                max
            })
            .min()
            .unwrap();

        println!("{min_time}");
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
