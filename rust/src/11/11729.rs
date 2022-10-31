use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let mut count = 0;
    // let mut tower: [Vec<i32>; 3] = [(1..=n).rev().collect(), Vec::new(), Vec::new()];
    let mut order: Vec<(usize, usize)> = Vec::new();

    hanoi(n, 0, 2, &mut count, &mut order);

    writeln!(stdout, "{count}").unwrap();

    for (a, b) in order {
        writeln!(stdout, "{a} {b}").unwrap();
    }
}

fn hanoi(n: i32, src: usize, dst: usize, count: &mut i32, order: &mut Vec<(usize, usize)>) {
    if n == 0 {
        return;
    }

    *count += 1;

    let child_dst = if (src, dst) == (0, 1) || (src, dst) == (1, 0) {
        2
    } else if (src, dst) == (0, 2) || (src, dst) == (2, 0) {
        1
    } else {
        0
    };

    hanoi(n - 1, src, child_dst, count, order);

    //let top = tower[src].pop().unwrap();
    //tower[dst].push(top);
    // println!("{tower:?}");
    order.push((src + 1, dst + 1));

    hanoi(n - 1, child_dst, dst, count, order);
}
