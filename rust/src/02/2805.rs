use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();

    if let [_, m] = parse_int_vec(lines.next().unwrap())[..] {
        let heights = parse_int_vec(lines.next().unwrap());
        let max_height = *heights.iter().max().unwrap();

        writeln!(stdout, "{}", binary_search(&heights, m, 0, max_height - 1)).unwrap();
    }
}

fn binary_search(heights: &Vec<i64>, m: i64, start: i64, end: i64) -> i64 {
    // println!("{start} {} {end}", (start + end) / 2);
    let is_ok = |num: i64| heights.iter().map(|h| (h - num).max(0)).sum::<i64>() >= m;

    if is_ok(end) {
        return end;
    }
    if end - start == 1 {
        return start;
    }

    let mid = (start + end) / 2;

    if is_ok(mid) {
        binary_search(heights, m, mid, end)
    } else {
        binary_search(heights, m, start, mid)
    }
}

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}

fn parse_int_vec(buf: &str) -> Vec<i64> {
    buf.split_whitespace().map(parse_int).collect()
}
