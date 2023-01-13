use std::collections::BTreeMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    for _ in 0..parse_int(input()) {
        let n = parse_int(input());
        let mut map = BTreeMap::new();

        for (op, num) in (0..n).map(|_| (input(), parse_int(input()))) {
            match op {
                "I" => {
                    map.entry(num).and_modify(|c| *c += 1).or_insert(1);
                }
                "D" => {
                    if map.is_empty() {
                        continue;
                    }

                    let key = *(if num == -1 {
                        map.keys().nth(0)
                    } else {
                        map.keys().last()
                    })
                    .unwrap();

                    if let Some(1) = map.get(&key) {
                        map.remove(&key);
                    } else {
                        map.entry(key).and_modify(|c| *c -= 1);
                    }
                }
                _ => (),
            }
            // println!("{map:?}");
        }

        if map.is_empty() {
            println!("EMPTY");
            continue;
        }

        let (min, max) = (map.keys().nth(0).unwrap(), map.keys().last().unwrap());

        println!("{max} {min}");
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
