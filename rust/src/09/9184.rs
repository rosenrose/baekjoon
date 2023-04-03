use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let mut memo = HashMap::new();

    while let (Some(a), Some(b), Some(c)) = (input.next(), input.next(), input.next()) {
        if (a, b, c) == (-1, -1, -1) {
            break;
        }

        writeln!(output, "w({a}, {b}, {c}) = {}", w(a, b, c, &mut memo)).unwrap();
    }

    print!("{output}");
}

fn w(a: i32, b: i32, c: i32, memo: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
    if a <= 0 || b <= 0 || c <= 0 {
        return 1;
    }

    let mut get_or_insert = |a: i32, b: i32, c: i32| {
        if let Some(i) = memo.get(&(a, b, c)) {
            *i
        } else {
            let ret = w(a, b, c, memo);
            memo.insert((a, b, c), ret);

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
