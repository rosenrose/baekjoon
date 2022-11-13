use std::collections::HashMap;
use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut table = HashMap::new();

    buf.lines().for_each(|line| {
        if line == "-1 -1 -1" {
            return;
        }

        if let [a, b, c] = parse_int_vec(line)[..] {
            writeln!(stdout, "w({a}, {b}, {c}) = {}", w(a, b, c, &mut table)).unwrap();
        }
    });
}

fn w(a: i32, b: i32, c: i32, table: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
    if a <= 0 || b <= 0 || c <= 0 {
        return 1;
    }

    let mut get_or_insert = |(a, b, c): (i32, i32, i32)| match table.get(&(a, b, c)) {
        Some(i) => *i,
        None => {
            let ret = w(a, b, c, table);
            table.insert((a, b, c), ret);

            ret
        }
    };

    if a > 20 || b > 20 || c > 20 {
        return get_or_insert((20, 20, 20));
    }

    if a < b && b < c {
        return get_or_insert((a, b, c - 1)) + get_or_insert((a, b - 1, c - 1))
            - get_or_insert((a, b - 1, c));
    }

    get_or_insert((a - 1, b, c))
        + get_or_insert((a - 1, b - 1, c))
        + get_or_insert((a - 1, b, c - 1))
        - get_or_insert((a - 1, b - 1, c - 1))
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
