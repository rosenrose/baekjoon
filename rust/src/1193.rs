fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let diagonal_len = |num| num;

    let mut step = 1;
    let mut diagonal_len_accum = diagonal_len(step);

    while diagonal_len_accum < n {
        step += 1;
        diagonal_len_accum += diagonal_len(step);
    }

    let (x, y) = if step % 2 == 0 { (step, 1) } else { (1, step) };
    let offset = diagonal_len_accum - n;

    let (x, y) = if step % 2 == 0 {
        (x - offset, y + offset)
    } else {
        (x + offset, y - offset)
    };

    println!("{x}/{y}");
}
