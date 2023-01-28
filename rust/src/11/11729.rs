use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let n: u32 = buf.trim().parse().unwrap();

    // let mut tower: [Vec<i32>; 3] = [(1..=n).rev().collect(), Vec::new(), Vec::new()];
    writeln!(output, "{}", (1 << n) - 1).unwrap();

    hanoi(n, 1, 3, &mut output);

    print!("{output}");
}

fn hanoi(n: u32, src: usize, dst: usize, output: &mut String) {
    if n == 0 {
        return;
    }

    let child_dst = match (src, dst) {
        (1, 2) | (2, 1) => 3,
        (1, 3) | (3, 1) => 2,
        (2, 3) | (3, 2) => 1,
        _ => 0,
    };

    hanoi(n - 1, src, child_dst, output);

    //let top = tower[src].pop().unwrap();
    //tower[dst].push(top);
    // println!("{tower:?}");
    writeln!(output, "{src} {dst}").unwrap();

    hanoi(n - 1, child_dst, dst, output);
}
