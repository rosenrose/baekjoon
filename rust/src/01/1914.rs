use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: u32 = buf.trim().parse().unwrap();

    let mut count = 0;
    let mut order: Vec<(usize, usize)> = Vec::new();

    if n > 20 {
        writeln!(stdout, "{}", 2_i128.pow(n) - 1).unwrap();
        return;
    }

    hanoi(n, 0, 2, &mut count, &mut order);

    writeln!(stdout, "{count}").unwrap();

    for (a, b) in order {
        writeln!(stdout, "{a} {b}").unwrap();
    }
}

fn hanoi(n: u32, src: usize, dst: usize, count: &mut i32, order: &mut Vec<(usize, usize)>) {
    if n == 0 {
        return;
    }

    *count += 1;

    let child_dst = match (src, dst) {
        (0, 1) | (1, 0) => 2,
        (0, 2) | (2, 0) => 1,
        (1, 2) | (2, 1) => 0,
        _ => 0,
    };

    hanoi(n - 1, src, child_dst, count, order);

    order.push((src + 1, dst + 1));

    hanoi(n - 1, child_dst, dst, count, order);
}
