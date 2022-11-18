fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [k, n] = parse_int_vec(&buf)[..] {
        let cables = parse_int_vec_lines(&mut buf, k);
        let max_length = *cables.iter().max().unwrap();

        println!("{}", binary_search(&cables, n, 1, max_length));
    }
}

fn binary_search(cables: &Vec<i64>, n: i64, start: i64, end: i64) -> i64 {
    // println!("{start} {} {end}", (start + end) / 2);
    let is_ok = |num| cables.iter().map(|len| len / num).sum::<i64>() >= n;

    if is_ok(end) {
        return end;
    }
    if end - start == 1 {
        return start;
    }

    let mid = (start + end) / 2;

    if is_ok(mid) {
        binary_search(cables, n, mid, end)
    } else {
        binary_search(cables, n, start, mid)
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(parse_int).collect()
}

fn parse_int_vec_lines(buf: &mut String, n: i64) -> Vec<i64> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_int(buf.trim())
        })
        .collect()
}
