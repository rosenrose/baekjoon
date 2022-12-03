fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        let sum: u32 = ('A'..='Z')
            .filter_map(|c| (!buf.contains(c)).then(|| c as u32))
            .sum();

        println!("{sum}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
