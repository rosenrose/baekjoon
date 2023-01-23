fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b] = parse_int_vec(&buf)[..] else { return };
    let sequence = (0..).scan((0, 1), |(num, counter), _| {
        if *counter == 1 {
            *num += 1;
            *counter = *num;
        } else {
            *counter -= 1;
        }

        Some(*num)
    });

    println!("{}", sequence.skip(a - 1).take(b - a + 1).sum::<i32>());
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
