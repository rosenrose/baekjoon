use std::collections::BTreeMap;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    for _ in 0..parse_int(input()) {
        let n = parse_int(input());
        let mut map = BTreeMap::new();

        for _ in 0..n {
            let (op, num) = (input(), parse_int(input()));

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

                    match map.get(&key).unwrap() {
                        1 => {
                            map.remove(&key);
                        }
                        value => {
                            map.insert(key, value - 1);
                        }
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
