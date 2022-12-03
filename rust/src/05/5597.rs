fn main() {
    let mut buf = String::new();
    const N: usize = 30;

    let mut is_submit = [false; N];

    for _ in 0..N - 2 {
        read_line(&mut buf);
        let num: usize = buf.trim().parse().unwrap();

        is_submit[num - 1] = true;
    }

    let absent = is_submit
        .iter()
        .enumerate()
        .filter_map(|(num, &submit)| (!submit).then(|| num + 1));

    for num in absent {
        println!("{num}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
