use std::collections::HashMap;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let mut cache = HashMap::new();

    loop {
        let (a, b, c) = (
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        );

        if (a, b, c) == (-1, -1, -1) {
            break;
        }

        writeln!(output, "w({a}, {b}, {c}) = {}", w(a, b, c, &mut cache)).unwrap();
    }

    print!("{output}");
}

fn w(a: i32, b: i32, c: i32, cache: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
    if a <= 0 || b <= 0 || c <= 0 {
        return 1;
    }

    let mut get_or_insert = |a: i32, b: i32, c: i32| match cache.get(&(a, b, c)) {
        Some(i) => *i,
        None => {
            let ret = w(a, b, c, cache);
            cache.insert((a, b, c), ret);

            ret
        }
    };

    if a > 20 || b > 20 || c > 20 {
        return get_or_insert(20, 20, 20);
    }

    if a < b && b < c {
        return get_or_insert(a, b, c - 1) + get_or_insert(a, b - 1, c - 1)
            - get_or_insert(a, b - 1, c);
    }

    get_or_insert(a - 1, b, c) + get_or_insert(a - 1, b - 1, c) + get_or_insert(a - 1, b, c - 1)
        - get_or_insert(a - 1, b - 1, c - 1)
}
